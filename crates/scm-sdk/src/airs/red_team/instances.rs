use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result, engine::AuthKind};
use http::Method;
#[derive(Clone, Debug)]
pub struct RedTeamInstancesClient {
    pub(super) client: Client,
}
impl RedTeamInstancesClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub async fn create_instance(
        &self,
        body: &InstanceRequest,
        options: Option<&RequestOptions>,
    ) -> Result<InstanceResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/instances")
                    .named("airs.red_team.instances.create_instance"),
                body,
                "InstanceRequestSchema",
                "InstanceResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_instance(
        &self,
        tenant_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<InstanceGetResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/instances")
                    .segment(tenant_id)?
                    .named("airs.red_team.instances.get_instance"),
                "InstanceGetResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_instance(
        &self,
        tenant_id: &str,
        body: &InstanceRequest,
        options: Option<&RequestOptions>,
    ) -> Result<InstanceResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/instances")
                    .segment(tenant_id)?
                    .named("airs.red_team.instances.update_instance"),
                body,
                "InstanceRequestSchema",
                "InstanceResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete_instance(
        &self,
        tenant_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<InstanceResponse> {
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/instances")
                    .segment(tenant_id)?
                    .named("airs.red_team.instances.delete_instance"),
                "InstanceResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_devices(
        &self,
        tenant_id: &str,
        body: &DeviceRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DeviceResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/instances")
                    .segment(tenant_id)?
                    .segment("devices")?
                    .named("airs.red_team.instances.create_devices"),
                body,
                "DeviceRequestSchema",
                "DeviceResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_devices(
        &self,
        tenant_id: &str,
        body: &DeviceRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DeviceResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PATCH, "/v1/instances")
                    .segment(tenant_id)?
                    .segment("devices")?
                    .named("airs.red_team.instances.update_devices"),
                body,
                "DeviceRequestSchema",
                "DeviceResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete_devices(
        &self,
        tenant_id: &str,
        serial_numbers: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DeviceResponse> {
        common::filter(serial_numbers)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/instances")
                    .segment(tenant_id)?
                    .segment("devices")?
                    .query([("serial_numbers".into(), serial_numbers.into())])
                    .named("airs.red_team.instances.delete_devices"),
                "DeviceResponseSchema",
                options,
            )
            .await
    }
    /// This POST may mint credentials, so it is never automatically replayed.
    pub async fn get_registry_credentials(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<RegistryCredentials> {
        self.client
            .send_contract(
                RawRequest::new(Method::POST, "/v1/registry-credentials")
                    .named("airs.red_team.instances.get_registry_credentials"),
                "RegistryCredentialsSchema",
                options,
            )
            .await
    }
}
