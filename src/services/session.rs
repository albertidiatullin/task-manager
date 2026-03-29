use crate::{exceptions::exception::AppError, repository::{session::SessionRepository, user::UserRepository}};
use chrono::{Utc,DateTime};


#[derive(Clone)]
pub struct SessionService{
    pub user_repo:UserRepository,
    pub session_repo:SessionRepository
}

impl SessionService {
    pub async fn session_valid_check<'a>(&self,get_uuid:&'a str) -> Result<(), AppError>{
        let utc_time_now = Utc::now();
        let model = self.user_repo.convert(get_uuid).await.map_err(|e| AppError::InternalServerError { err: e.to_string() })?;
        let actual_user_name = self.user_repo.get_user_of_uuid(model).await.map_err(|_e| AppError::UserNotFound)?;
        let actual_session = self.session_repo.get_session(&actual_user_name.user_name).await.
        map_err(|_e|AppError::UserNotFound)?.ok_or(AppError::UserNotFound)?;
        let parsed_time = DateTime::parse_from_rfc3339(&actual_session.session_experation).
        map_err(|_e|AppError::ParseTimeError)?;
        let utc_parsed_time:DateTime<Utc> = parsed_time.to_utc();

        if utc_time_now>utc_parsed_time{
            self.session_repo.delete_session(get_uuid.to_string()).await.map_err(|_e| AppError::DBErorr)?;
            return Err(AppError::SessionNotValid);
        }
        Ok(())
    }
}