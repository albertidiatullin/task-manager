use crate::dto::user_dto::{RegisterDTO,AuthDTO};
use sqlx::prelude::FromRow;
use sqlx::{ sqlite::SqlitePool};

use sqlx::{query};
use bcrypt::{hash,DEFAULT_COST};
use tracing::error;

#[derive(Clone)]
pub struct UserRepository{
    pub sqlite_pool:SqlitePool
}

#[derive(FromRow)]
pub struct ResponseDomainModel{
    pub user_id:i32,
    pub user_name:String,
    pub user_password:String,
    pub user_uuid:String,
}


pub struct GetUserModel<'a>{
    pub user_name:&'a str,
    pub user_password:&'a str
}
pub struct GetUserUUIDModel{
    pub user_uuid:String
}

pub struct DataForHandler{
    pub user_id:i32,
    pub user_name:String,
    pub user_password:String,
    pub user_uuid:String

}
impl From<ResponseDomainModel> for DataForHandler {
    fn from(value: ResponseDomainModel) -> Self {
        Self { user_id:value.user_id,
            user_name:value.user_name, 
            user_password: value.user_password ,
            user_uuid:value.user_uuid}
    }
}


impl From<RegisterDTO> for AuthDTO {
    fn from(value: RegisterDTO) -> Self {
        Self { user_name:value.user_name,
             user_password: value.user_password }
    }
    
}

impl UserRepository {
    pub async fn add_user(&self,data:RegisterDTO,user_uuid:&String) -> Option<bool>{
        let hashed_password = hash(&data.user_password, DEFAULT_COST).unwrap();
        let data = query("INSERT INTO users (user_name,user_uuid,user_email,user_password) VALUES(?,?,?,?)")
        .bind(&data.user_name)
        .bind(user_uuid)
        .bind(&data.user_email)
        .bind(&hashed_password)
        .execute(&self.sqlite_pool).await;

        if let Err(err) = data{
            error!("{:?}",err);
        }
        Some(true)


    }

    pub async fn get_user_of_name<'a>(&self,data:&GetUserModel<'a>) -> Option<DataForHandler>{
    let query  = sqlx::query_as::<_,ResponseDomainModel>(
        "SELECT user_id,users.user_name,user_password,session.user_uuid FROM users INNER JOIN 
        session ON session.user_name = users.user_name WHERE users.user_name = ?;")
        .bind(data.user_name)
    .fetch_one(&self.sqlite_pool)
    .await;

    if let Ok(query_data) = query{
        let data_for_handler:DataForHandler = query_data.into();
        return Some(data_for_handler);
    }
    
    None
    }

       pub async fn get_user_of_uuid(&self,data:GetUserUUIDModel) -> anyhow::Result<DataForHandler>{
    let query  = sqlx::query_as::<_,ResponseDomainModel>(
        "SELECT user_id,user_name,user_password,user_uuid FROM users WHERE user_uuid=?")
        .bind(data.user_uuid)
    .fetch_one(&self.sqlite_pool)
    .await;
    if let Ok(query_data) = query{
        let data_for_handler:DataForHandler = query_data.into();
        return Ok(data_for_handler);
    }
    
    Err(anyhow::Error::msg("Пользователь не найден"))
    }

    pub async fn get_user_for_create_task<'a>(&self,model:GetUserUUIDModel) -> Option<DataForHandler>{
        let user_data = self.get_user_of_uuid(model).await;
        if let Ok(data) = user_data{
            return Some(data);
        }
        None
    }


    pub async fn convert<'a>(&self,get_uuid:&'a str) -> anyhow::Result<GetUserUUIDModel> {
        let model:GetUserUUIDModel = GetUserUUIDModel {
            user_uuid:get_uuid.to_string()
        };

    Ok(model)
    }

    pub async fn get_user_of_uuid_use_in_service(&self,model:GetUserUUIDModel) -> anyhow::Result<DataForHandler>{
        let check_name = self.
        get_user_of_uuid(model).await;
        check_name
    }

    }




