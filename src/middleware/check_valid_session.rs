use axum::middleware::Next;
use axum::extract::Request;
use axum::response::Response;
use axum::extract::State;
use tower_cookies::{Cookie,Cookies};
use crate::core::state::AppState;
use crate::exceptions::exception::AppError;
use std::sync::Arc;


pub async fn session_valid_middleware(
                                    State(state):State<Arc<AppState>>,
                                    cookie:Cookies,
                                    req:Request,
                                    next:Next) -> Result<Response,AppError>{
    let uuid_cookie = cookie.get("uuid").ok_or(AppError::UserNotFound)?; 
    let _data = state.session_service.session_valid_check(uuid_cookie.value()).await.
    map_err(|_e| {cookie.remove(Cookie::new("uuid", ""));
                                   AppError::SessionNotValid});     
                      
    let response = next.run(req).await;
    Ok(response)
}