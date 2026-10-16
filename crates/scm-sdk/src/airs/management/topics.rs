use super::{PaginationOptions, models::*, next_offset, nonblank, optional_items, tenant, uuid};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::Optional,
    engine::AuthKind,
    pagination::{CollectOptions, CollectionResult, Page, Pages},
};
use http::Method;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct TopicListOptions {
    pub offset: u64,
    pub limit: u32,
    pub latest_only: bool,
}
impl Default for TopicListOptions {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
            latest_only: false,
        }
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TopicListAllOptions {
    pub limit: u32,
    pub collect: CollectOptions,
}
impl Default for TopicListAllOptions {
    fn default() -> Self {
        Self {
            limit: 100,
            collect: CollectOptions::default(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct TopicsClient {
    pub(super) client: Client,
}
impl TopicsClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn list_for_token(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomTopicListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/topics")
                    .named("airs.management.topics.list_for_token")
                    .query(page.query(false)?),
                "CustomTopicListResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &CreateCustomTopicRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomTopic> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/mgmt/topic")
                    .named("airs.management.topics.create"),
                body,
                "CreateCustomTopicRequestSchema",
                "CustomTopicSchema",
                options,
            )
            .await
    }
    async fn list_raw(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomTopicListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/topics/tsg")
                    .segment(tenant(&self.client)?)?
                    .named("airs.management.topics.list")
                    .query(page.query(false)?),
                "CustomTopicListResponseSchema",
                options,
            )
            .await
    }
    /// Latest revision selection preserves first-seen name order and first revision ties.
    pub async fn list(
        &self,
        page: TopicListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomTopicListResponse> {
        let pagination = PaginationOptions {
            offset: page.offset,
            limit: page.limit,
            latest: None,
        };
        pagination.query(false)?;
        if !page.latest_only {
            return self.list_raw(pagination, options).await;
        }
        let options = self.client.operation_options(options)?;
        let collection = self
            .list_all(
                TopicListAllOptions {
                    limit: 200,
                    ..TopicListAllOptions::default()
                },
                Some(&options),
            )
            .await
            .map_err(|error| error.into_error())?;
        if !collection.is_complete() {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut positions = BTreeMap::<String, usize>::new();
        let mut topics = Vec::<CustomTopic>::new();
        for topic in collection.items {
            if let Some(index) = positions.get(&topic.topic_name) {
                if topic.revision.as_f64() > topics[*index].revision.as_f64() {
                    topics[*index] = topic;
                }
            } else {
                positions.insert(topic.topic_name.clone(), topics.len());
                topics.push(topic);
            }
        }
        let offset = usize::try_from(page.offset).map_err(|_| Error::new(ErrorKind::Limit))?;
        let end = offset.saturating_add(page.limit as usize);
        let total = topics.len();
        let selected = topics
            .into_iter()
            .skip(offset)
            .take(page.limit as usize)
            .collect();
        let mut response = CustomTopicListResponse::builder()
            .custom_topics(Optional::Value(selected))
            .build()?;
        if end < total {
            response.next_offset = Optional::Value((end as u64).into());
        }
        Ok(response)
    }
    pub fn pages(
        &self,
        page: PaginationOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, CustomTopic, u64>> {
        page.query(false)?;
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
                        .list_raw(PaginationOptions { offset, ..page }, Some(&options))
                        .await?;
                    Ok(Page {
                        items: optional_items(response.custom_topics),
                        next: next_offset(response.next_offset)?,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        all: TopicListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<CustomTopic, u64> {
        self.pages(
            PaginationOptions {
                limit: all.limit,
                ..PaginationOptions::default()
            },
            all.collect.max_pages,
            options,
        )?
        .collect(all.collect)
        .await
    }
    pub async fn get(&self, id: &str, options: Option<&RequestOptions>) -> Result<CustomTopic> {
        let options = self.client.operation_options(options)?;
        let collection = self
            .list_all(TopicListAllOptions::default(), Some(&options))
            .await
            .map_err(|error| error.into_error())?;
        if !collection.is_complete() {
            return Err(Error::new(ErrorKind::Limit));
        }
        collection
            .items
            .into_iter()
            .find(|topic| matches!(&topic.topic_id,Optional::Value(value) if value==id))
            .ok_or_else(|| Error::new(ErrorKind::Validation))
    }
    pub async fn get_by_name(
        &self,
        name: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomTopic> {
        let options = self.client.operation_options(options)?;
        let collection = self
            .list_all(TopicListAllOptions::default(), Some(&options))
            .await
            .map_err(|error| error.into_error())?;
        if !collection.is_complete() {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut best: Option<CustomTopic> = None;
        for topic in collection
            .items
            .into_iter()
            .filter(|topic| topic.topic_name == name)
        {
            if best
                .as_ref()
                .is_none_or(|current| topic.revision.as_f64() > current.revision.as_f64())
            {
                best = Some(topic);
            }
        }
        best.ok_or_else(|| Error::new(ErrorKind::Validation))
    }
    pub async fn update(
        &self,
        id: &str,
        body: &CreateCustomTopicRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomTopic> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/mgmt/topic/uuid")
                    .segment(id)?
                    .named("airs.management.topics.update"),
                body,
                "CreateCustomTopicRequestSchema",
                "CustomTopicSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DeleteTopicResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/mgmt/topic")
                    .segment(id)?
                    .named("airs.management.topics.delete"),
                "DeleteTopicResponseSchema",
                options,
            )
            .await
    }
    pub async fn force_delete_with_audit(
        &self,
        id: &str,
        updated_by: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DeleteTopicResponse> {
        uuid(id)?;
        nonblank(updated_by)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/mgmt/topic")
                    .segment(id)?
                    .segment("force")?
                    .named("airs.management.topics.force_delete_with_audit")
                    .query([("updated_by".into(), updated_by.into())]),
                "DeleteTopicResponseSchema",
                options,
            )
            .await
    }
    pub async fn force_delete(
        &self,
        id: &str,
        updated_by: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<DeleteTopicResponse> {
        self.force_delete_with_audit(id, updated_by.unwrap_or(""), options)
            .await
    }
}
