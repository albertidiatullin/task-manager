use std::sync::Arc;
use crate::core::state::AppState;
use crate::exceptions::exception::AppError;
use axum::routing::{get, put,delete};
use axum::{Router,routing::post};
use crate::handlers::fallback::fallback_handler;
use crate::handlers::users::{register_handler,logout_handler,auth_handler};
use crate::handlers::tasks::{crate_task,get_all_tasks,get_task,edit_task,delete_task};
use crate::middleware::check_auth::auth_middleware;
use crate::middleware::check_valid_session::session_valid_middleware;
use crate::core::state::get_state;
use axum::middleware::{from_fn, from_fn_with_state};


pub async fn return_router() -> Router<Arc<AppState>>{
       
       Router::new()
       .route("/users/register", post(register_handler))
       .route("/users/auth", post(auth_handler))

       .fallback(fallback_handler)
}


pub async fn need_auth_return_router()-> Result<Router<Arc<AppState>>,AppError>{
       let state = get_state().await.map_err(|e|AppError::InternalServerError { err: e.to_string() })?;
       Ok(Router::new()
        .route("/users/me/logout", post(logout_handler))
        .route("/tasks/create_task", post(crate_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/{task_id}", get(get_task))
        .route("/tasks/edit", put(edit_task))
        .route("/tasks/delete", delete(delete_task))
        .layer(from_fn(auth_middleware))
        .layer(from_fn_with_state(state,session_valid_middleware)))

}