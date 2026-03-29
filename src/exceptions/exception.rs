

use axum::extract::Json;
use serde_json::json;
use crate::response::user_response::RegisterAuthReposnse;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use thiserror;
use thiserror::Error;


#[derive(Error,Debug)]
pub enum AppError {
    #[error("Пользователь не найден")]
    UserNotFound,
    #[error("Неверный пароль")]
    NotGodPassword,
    #[error("Ошибка регестрации")]
    RegisterErorr,
    #[error("Ошибка базы данных")]
    DBErorr,
    #[error("Пользователь уже авторизован")]
    ForribenUser,
    #[error("Пользователь не в аккаунте")]
    NotInAccount,
    #[error("Ошибка сервера")]
    InternalServerError {err:String},
    #[error("задачи не найдены")]
    TaskNotFound,
    #[error("Ошибка парсинга времени")]
    ParseTimeError,
    #[error("Сессия не валидна")]
    SessionNotValid
}


impl IntoResponse  for AppError{
    fn into_response(self) -> axum::response::Response {
        let data_response = RegisterAuthReposnse {
                message: "Произошла ошибка при регистрации"};

        let db_response_error = RegisterAuthReposnse {
                message: "Произошла ошибка при регистрации",};

        let error_parse_time = RegisterAuthReposnse{
            message:"Ошибка парсинга времени"
        };

        let error_session_experation = RegisterAuthReposnse{
            message:"Сессия просрочена"
        };
        match self {
            Self::SessionNotValid =>{
                (StatusCode::UNAUTHORIZED,Json(error_session_experation)).into_response()  
            },
            Self::ParseTimeError =>{
                tracing::error!("Ошибка парсинга времени сесси");
                (StatusCode::BAD_REQUEST,Json(error_parse_time)).into_response()
            },
            Self::NotGodPassword => {
                tracing::error!("NotGodPassword: неверный пароль");
                (StatusCode::FORBIDDEN, "Не верный пароль").into_response()
            }
            Self::UserNotFound => {
                tracing::error!("UserNotFound: пользователь не найден");
                (StatusCode::NOT_FOUND, "Пользователь не найден").into_response()
            }
            Self::RegisterErorr => {
                tracing::error!("RegisterErorr: ошибка регистрации, data_response = {:?}", data_response);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(data_response)).into_response()
            }
            Self::DBErorr => {
                tracing::error!("DBErorr: ошибка базы данных, db_response_error = {:?}", db_response_error);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(db_response_error)).into_response()
            }
            Self::ForribenUser => {
                tracing::error!("ForribenUser: пользователь уже авторизован");
                (
                    StatusCode::FORBIDDEN,
                    Json(json!({"message":"Пользователь уже авторизован"}))
                ).into_response()
            }
            Self::NotInAccount => {
                tracing::error!("NotInAccount: пользователь не в аккаунте");
                (
                    StatusCode::FORBIDDEN,
                    Json(json!({"message":"Вы не в аккаунте"}))
                ).into_response()
            }
            Self::InternalServerError{err} => {
                tracing::error!("InternalServerError: {:}",err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message":"ошибка сервера"}))
                ).into_response()
            },
            Self::TaskNotFound => {
                (StatusCode::NOT_FOUND,Json(json!({"задачи":"не найдено"}))).into_response()
            }
        }
            }
}