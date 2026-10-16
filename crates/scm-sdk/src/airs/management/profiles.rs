use super::models::SecurityProfileListResponse;
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result, engine::AuthKind};

/// Page boundaries are explicit. Whole-collection helpers retain truncation evidence.
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
    fn query(self) -> Result<Vec<(String, String)>> {
        if self.limit == 0 || self.offset > 9_007_199_254_740_991 {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut query = vec![
            ("offset".into(), self.offset.to_string()),
            ("limit".into(), self.limit.to_string()),
        ];
        if let Some(latest) = self.latest {
            query.push(("latest".into(), latest.to_string()));
        }
        Ok(query)
    }
}
#[derive(Clone, Debug)]
pub struct ManagementClient {
    client: Client,
}
impl ManagementClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn profiles(&self) -> ProfilesClient {
        ProfilesClient {
            client: self.client.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct ProfilesClient {
    client: Client,
}
impl ProfilesClient {
    /// List profiles using the tenant authorized by the OAuth token.
    pub async fn list_for_token(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<SecurityProfileListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/profiles")
                    .named("airs.management.profiles.list_for_token")
                    .query(page.query()?),
                "SecurityProfileListResponseSchema",
                options,
            )
            .await
    }
}
