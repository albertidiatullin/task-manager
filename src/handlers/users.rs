use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use tower_cookies::{Cookie, Cookies};


use crate::{core::state::AppState, dto::user_dto::{AuthDTO, RegisterDTO}, 
    exceptions::exception::AppError,
    response::user_response::RegisterAuthReposnse};

pub async fn register_handler<'a>(  cookie:Cookies,
                                State(state):State<Arc<AppState>>,
                                Json(payload):Json<RegisterDTO>)
                                -> Result<(StatusCode,Json<RegisterAuthReposnse<'a>>),AppError>{
        let register_data = state.user_service.register_user(payload).await?;

        let response_data = RegisterAuthReposnse{
            message:"Вы успешно зарегитсривровались"
        };
        let mut new_cookie = Cookie::new("uuid", register_data.to_string());
        new_cookie.set_http_only(true);
        new_cookie.set_path("/");
        cookie.add(new_cookie);
        Ok((StatusCode::CREATED,Json(response_data)))
        }

       


pub async fn logout_handler<'a>(
    cookie:Cookies,
    State(state):State<Arc<AppState>>
) -> Result<(StatusCode,Json<RegisterAuthReposnse<'a>>),AppError>{
    let c_uuid = cookie.get("uuid").ok_or(AppError::NotInAccount)?;
    state.session_repo.delete_session(c_uuid.value().to_string()). await.map_err(|e| AppError::InternalServerError { err: e.to_string() })?;
    cookie.remove(Cookie::new("uuid", ""));
   
    let response_return = RegisterAuthReposnse{
        message:"Вы вышли с аккаунта"
    };

    Ok((StatusCode::OK,Json(response_return)))
}


pub async fn auth_handler<'a>(cookie:Cookies,
                            State(state):State<Arc<AppState>>,
                            Json(payload):Json<AuthDTO>) -> Result<(StatusCode,Json<RegisterAuthReposnse<'a>>),AppError>{
      

        let auth_data = state.user_service.auth_handler(payload).await?;
        let mut new_cookie = Cookie::new("uuid", auth_data.to_string());
        new_cookie.set_http_only(true);
        new_cookie.set_path("/");
        cookie.add(new_cookie);
        let response_handler = RegisterAuthReposnse{
            message:"Вы успешно авторизовались"};


        Ok((StatusCode::OK,Json(response_handler)))

    }