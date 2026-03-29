
use axum::extract::Request;
use axum::middleware::Next;
use tracing::info;
use crate::exceptions::exception::AppError;
use axum::response::Response;
use tower_cookies::{Cookies};


pub async fn auth_middleware(cookie:Cookies,req:Request,next:Next) -> Result<Response,AppError>{
    
    let response = next.run(req).await;
    let value = cookie.get("uuid").ok_or(AppError::NotInAccount)?;
    info!("Cookie value {:}",value.to_string());
    Ok(response)
    

    }


