use std::sync::Arc;
use crate::core::state::AppState;
use crate::repository::tasks::{TaskModelDomain};
use crate::response::user_response::RegisterAuthReposnse;
use axum::extract::{State,Json};
use axum::http::StatusCode;
use axum::extract::Path;
use crate::dto::task::{CreateTaskDTO, EditDTO};
use crate::exceptions::exception::AppError;
use tower_cookies::Cookies;

pub async fn crate_task<'a>(State(state): State<Arc<AppState>>,
                        cookie:Cookies,
                        Json(payload):Json<CreateTaskDTO>,
                    ) -> Result<(StatusCode,Json<RegisterAuthReposnse<'a>>),AppError>{
    let get_uuid_cookie = cookie.get("uuid").ok_or(AppError::UserNotFound)?;
    state.task_service.create_task(payload, get_uuid_cookie.value()).await?;
    let data = RegisterAuthReposnse{
        message:"Вы успешно создали задачу",
    };
    Ok((StatusCode::CREATED,Json(data)))

}


pub async fn get_all_tasks(State(state): State<Arc<AppState>>,
                        cookie:Cookies)-> Result<(StatusCode,Json<Vec<TaskModelDomain>>),AppError>{
    let get_uuid_cookie = cookie.get("uuid").ok_or(AppError::UserNotFound)?;
    let all_tasks = state.task_service.get_all_tasks(get_uuid_cookie.value()).await?;
    Ok((StatusCode::OK,Json(all_tasks)))

}


pub async fn get_task(State(state): State<Arc<AppState>>,
                    cookie:Cookies,
                    Path(task_id):Path<String>)-> Result<(StatusCode,Json<TaskModelDomain>),AppError>{
    let get_uuid_cookie = cookie.get("uuid").ok_or(AppError::UserNotFound)?;
    let task = state.task_service.get_task_by_id(get_uuid_cookie.value(), &task_id).await?;
    Ok((StatusCode::ACCEPTED,Json(task)))
    
}


pub async fn edit_task<'a>(cookie:Cookies,
                    State(state): State<Arc<AppState>>,
                    Json(payload):Json<EditDTO>) -> Result<(StatusCode,Json<RegisterAuthReposnse<'a>>),AppError>{
    let get_uuid = cookie.get("uuid").ok_or(AppError::NotInAccount)?;
    state.task_service.edit_task(get_uuid.value(), &payload).await?;
    let response = RegisterAuthReposnse{
        message:"Вы успешно изменили задачу"
    };
    Ok((StatusCode::OK,Json(response)))
}

pub async fn delete_task<'a>(cookie:Cookies,
                        State(state):State<Arc<AppState>>,
                        Path(task_id):Path<String>) -> Result<(StatusCode,Json<RegisterAuthReposnse<'a>>),AppError>{
    let get_uuid = cookie.get("uuid").ok_or(AppError::UserNotFound)?;
    state.task_service.delete_task(&task_id, get_uuid.value()).await.map_err(|e| 
        AppError::InternalServerError { err: e.to_string() })?;
    
    let repsonse = RegisterAuthReposnse { 
        message:"Вы успешно удалили задачу"
    };
    Ok((StatusCode::OK,Json(repsonse)))
    
}