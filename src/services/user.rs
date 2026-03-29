use crate::dto::user_dto::AuthDTO;
use crate::exceptions::exception::AppError;
use crate::repository::session::SessionRepository;
use crate::{dto::user_dto::RegisterDTO, repository::user::UserRepository};
use crate::repository::user::GetUserModel;
use uuid::Uuid;
use bcrypt::verify;

#[derive(Clone)]
pub struct UserService{
    pub user_repo:UserRepository,
    pub session_repo:SessionRepository
}

impl UserService {
    pub async fn register_user(&self,payload:RegisterDTO) -> Result<Uuid, AppError>{
         let model= GetUserModel
        {
            user_name:&payload.user_name.to_string(),
        user_password:&payload.user_password.to_string(),
    };
    let user_name_for_sesion = payload.user_name.clone();
    let data = self.user_repo.get_user_of_name(&model).await;
    if data.is_some(){
            return Err(AppError::ForribenUser);
        }
    let uuid= Uuid::new_v4();
    self.user_repo.add_user(payload,&uuid.to_string()).await.ok_or(AppError::DBErorr)?;
    
    self.session_repo.add_session(uuid.to_string(),&user_name_for_sesion).await.map_err(|e| AppError::InternalServerError{err:e.to_string()})?;
    Ok(uuid)
    }


    pub async fn auth_handler(&self,payload:AuthDTO) -> Result<Uuid, AppError>{
          let get_user_model = GetUserModel{
                user_name:&payload.user_name,
                user_password:&payload.user_password,
        };


        let get_user = self.user_repo.get_user_of_name(&get_user_model).await.ok_or(AppError::UserNotFound)?;
        let verfiriy_status = verify(&payload.user_password, &get_user.user_password).
        map_err(|e| AppError::InternalServerError{err:e.to_string()})?;
        let uuid_session = Uuid::new_v4();
        self.session_repo.add_session(uuid_session.to_string(), &payload.user_name).await.map_err(|e|AppError::InternalServerError{err:e.to_string()})?;
        if !verfiriy_status{
            return Err(AppError::NotGodPassword);
        }

        Ok(uuid_session)
        
    }
}