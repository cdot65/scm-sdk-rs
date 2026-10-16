//! Network Broker channels use their explicitly configured, distinct endpoint.
use super::{
    common,
    models::{
        Channel, ChannelListResponse, ChannelStats, CreateChannelRequest, UpdateChannelRequest,
    },
};
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result};
use http::Method;
const PATH: &str = "/v1/channels";

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ChannelListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    /// Each status is sent separately in caller order, including duplicates.
    pub status: Vec<String>,
    pub include_all_if_empty: Option<bool>,
}
crate::redaction::redacted_debug!(ChannelListOptions);
#[derive(Clone, Debug)]
pub struct RedTeamNetworkBrokerClient {
    pub(super) client: Client,
}
impl RedTeamNetworkBrokerClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub async fn list_channels(
        &self,
        params: &ChannelListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ChannelListResponse> {
        if params.status.len() > 252 {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        for status in &params.status {
            common::filter(status)?;
            query.push(("status".into(), status.clone()));
        }
        if let Some(value) = params.include_all_if_empty {
            query.push(("include_all_if_empty".into(), value.to_string()));
        }
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(query)
                    .named("red_team.network_broker.list_channels"),
                "ChannelListResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_channel(
        &self,
        body: &CreateChannelRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Channel> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, PATH).named("red_team.network_broker.create_channel"),
                body,
                "CreateChannelRequestSchema",
                "ChannelSchema",
                options,
            )
            .await
    }
    pub async fn get_channel_stats(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<ChannelStats> {
        self.client
            .send_contract(
                RawRequest::read("/v1/channels/stats")
                    .named("red_team.network_broker.get_channel_stats"),
                "ChannelStatsSchema",
                options,
            )
            .await
    }
    pub async fn get_channel(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Channel> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(uuid)?
                    .named("red_team.network_broker.get_channel"),
                "ChannelSchema",
                options,
            )
            .await
    }
    /// Ordinary JSON PATCH; this endpoint does not use JSON merge-patch media type.
    pub async fn update_channel(
        &self,
        uuid: &str,
        body: &UpdateChannelRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Channel> {
        common::uuid(uuid)?;
        self.client
            .send_model(
                RawRequest::new(Method::PATCH, PATH)
                    .segment(uuid)?
                    .named("red_team.network_broker.update_channel"),
                body,
                "UpdateChannelRequestSchema",
                "ChannelSchema",
                options,
            )
            .await
    }
}
