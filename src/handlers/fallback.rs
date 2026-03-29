use axum::extract::{Json};
use axum::http::StatusCode;
use serde_json::json;
use serde_json::Value;

pub async fn fallback_handler() -> (StatusCode,Json<Value>){
        let response = json!({
        "status": "error",
        "code": 404,
        "message": "Route not found",
        "details": {
            "hint": "Check the URL or method",
        }
    });

    (StatusCode::NOT_FOUND, Json(response))
} 