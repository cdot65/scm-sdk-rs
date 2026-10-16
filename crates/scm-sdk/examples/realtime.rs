//! Explicit realtime transport and graceful cancellation. Requires a configured provider.
#[cfg(all(feature = "ws-transport", feature = "reqwest-transport"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> scm_sdk::Result<()> {
    use scm_sdk::{
        Client, Endpoint, Error, ErrorKind, Secret,
        airs::gateway::{
            RuntimeClient,
            models::{GatewayRealtimeConnectRequest, GatewayRealtimeEvent},
        },
        auth::Authentication,
        data::Optional,
        realtime::{GatewayRealtimeOptions, NativeWebSocketConnector},
        validation::Mode,
    };
    use std::sync::Arc;
    let read = |name: &str| std::env::var(name).map_err(|_| Error::new(ErrorKind::Configuration));
    let runtime = RuntimeClient::new(
        Client::builder(Endpoint::parse(&read("PANW_AI_GW_INFERENCE_ENDPOINT")?)?)
            .authentication(Authentication::runtime_key(Secret::new(read(
                "PANW_AI_GW_INFERENCE_API_KEY",
            )?)?)?)
            .build()?,
    )?;
    let query = GatewayRealtimeConnectRequest::builder()
        .model(Optional::Value(read("PANW_AI_GW_REALTIME_MODEL")?))
        .build_validated(Mode::Request)?;
    // Direct verified TLS is an explicit selection. HTTP proxy configuration is separate.
    let transport = GatewayRealtimeOptions::new(Arc::new(NativeWebSocketConnector::new()?));
    let mut connection = runtime.connect_realtime(&query, &transport, None).await?;
    // Upgrade success does not imply provider readiness. Unknown events are retained.
    let mut ready = false;
    while let Some(event) = connection.next().await? {
        if event.r#type == "error" {
            connection.cancel().await?;
            return Err(Error::new(ErrorKind::Protocol));
        }
        if event.r#type == "session.created" {
            ready = true;
            break;
        }
    }
    if !ready {
        return Err(Error::new(ErrorKind::Protocol));
    }
    let request = GatewayRealtimeEvent::builder()
        .r#type("response.create".to_owned())
        .build_validated(Mode::Request)?;
    connection.send(&request).await?;
    if let Some(event) = connection.next().await? {
        // Inspect event types for readiness/errors; do not print sensitive provider payloads.
        let _event_type = &event.r#type;
    }
    connection.cancel().await
}
#[cfg(not(all(feature = "ws-transport", feature = "reqwest-transport")))]
fn main() {}
