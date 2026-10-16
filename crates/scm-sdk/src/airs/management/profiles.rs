use super::{models::*, next_offset, optional_items, tenant, uuid};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::Optional,
    engine::AuthKind,
    pagination::{CollectOptions, CollectionResult, Page, Pages},
};
use http::Method;
/// Offset pagination used by AIRS resource inventories.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct PaginationOptions {
    pub offset: u64,
    pub limit: u32,
    pub latest: Option<bool>,
}
impl Default for PaginationOptions {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
            latest: None,
        }
    }
}
impl PaginationOptions {
    pub(crate) fn query(self, latest: bool) -> Result<Vec<(String, String)>> {
        if self.limit == 0 || self.offset > 9_007_199_254_740_991 {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut query = vec![
            ("offset".into(), self.offset.to_string()),
            ("limit".into(), self.limit.to_string()),
        ];
        if latest && let Some(value) = self.latest {
            query.push(("latest".into(), value.to_string()));
        }
        Ok(query)
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ProfileListAllOptions {
    pub limit: u32,
    pub latest: Option<bool>,
    pub collect: CollectOptions,
}
impl Default for ProfileListAllOptions {
    fn default() -> Self {
        Self {
            limit: 100,
            latest: None,
            collect: CollectOptions::default(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct ProfilesClient {
    pub(super) client: Client,
}
impl ProfilesClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn list_for_token(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<SecurityProfileListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/profiles")
                    .named("airs.management.profiles.list_for_token")
                    .query(page.query(true)?),
                "SecurityProfileListResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &CreateSecurityProfileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<SecurityProfile> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/mgmt/profile")
                    .named("airs.management.profiles.create"),
                body,
                "CreateSecurityProfileRequestSchema",
                "SecurityProfileSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<SecurityProfileListResponse> {
        let request = RawRequest::read("/v1/mgmt/profiles/tsg")
            .segment(tenant(&self.client)?)?
            .named("airs.management.profiles.list")
            .query(page.query(true)?);
        self.client
            .send_contract(request, "SecurityProfileListResponseSchema", options)
            .await
    }
    pub fn pages(
        &self,
        page: PaginationOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, SecurityProfile, u64>> {
        page.query(true)?;
        let client = self.clone();
        Pages::new(
            page.offset,
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |offset, options| {
                let client = client.clone();
                async move {
                    let response = client
                        .list(PaginationOptions { offset, ..page }, Some(&options))
                        .await?;
                    Ok(Page {
                        items: optional_items(response.ai_profiles),
                        next: next_offset(response.next_offset)?,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        all: ProfileListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<SecurityProfile, u64> {
        self.pages(
            PaginationOptions {
                limit: all.limit,
                latest: all.latest,
                ..PaginationOptions::default()
            },
            all.collect.max_pages,
            options,
        )?
        .collect(all.collect)
        .await
    }
    pub async fn get(&self, id: &str, options: Option<&RequestOptions>) -> Result<SecurityProfile> {
        let options = self.client.operation_options(options)?;
        let collection = self
            .list_all(ProfileListAllOptions::default(), Some(&options))
            .await
            .map_err(|error| error.into_error())?;
        if !collection.is_complete() {
            return Err(Error::new(ErrorKind::Limit));
        }
        collection
            .items
            .into_iter()
            .find(|profile| matches!(&profile.profile_id,Optional::Value(value) if value==id))
            .ok_or_else(|| Error::new(ErrorKind::Validation))
    }
    pub async fn get_by_name(
        &self,
        name: &str,
        options: Option<&RequestOptions>,
    ) -> Result<SecurityProfile> {
        let options = self.client.operation_options(options)?;
        let collection = self
            .list_all(ProfileListAllOptions::default(), Some(&options))
            .await
            .map_err(|error| error.into_error())?;
        if !collection.is_complete() {
            return Err(Error::new(ErrorKind::Limit));
        }
        let revision = |profile: &SecurityProfile| match &profile.revision {
            Optional::Value(value) => value.as_f64().unwrap_or(0.0),
            Optional::Unset => 0.0,
        };
        let mut best: Option<SecurityProfile> = None;
        for profile in collection
            .items
            .into_iter()
            .filter(|profile| profile.profile_name == name)
        {
            if best
                .as_ref()
                .is_none_or(|current| revision(&profile) > revision(current))
            {
                best = Some(profile);
            }
        }
        best.ok_or_else(|| Error::new(ErrorKind::Validation))
    }
    pub async fn update(
        &self,
        id: &str,
        body: &CreateSecurityProfileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<SecurityProfile> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/mgmt/profile/uuid")
                    .segment(id)?
                    .named("airs.management.profiles.update"),
                body,
                "CreateSecurityProfileRequestSchema",
                "SecurityProfileSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DeleteProfileResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/mgmt/profile")
                    .segment(id)?
                    .named("airs.management.profiles.delete"),
                "DeleteProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn force_delete(
        &self,
        id: &str,
        updated_by: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DeleteProfileResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/mgmt/profile")
                    .segment(id)?
                    .segment("force")?
                    .named("airs.management.profiles.force_delete")
                    .query([("updated_by".into(), updated_by.into())]),
                "DeleteProfileResponseSchema",
                options,
            )
            .await
    }
}
