use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn logging_middleware(req:Request,next:Next) -> Response{
    let method = req.method().clone();
    let uri = req.uri().path().to_string();
    info!("⚡ [UPLINK_ESTABLISHED] :: {method} >> {uri} | Monitoring data-stream...");
    let response = next.run(req).await;
    let status = response.status();
    info!("💾 [PACKET_SENT] :: {uri} << STATUS_CODE: {status} | Encryption: ACTIVE");

    response
}