use sqlx::SqlitePool;

use crate::core::settings::AppConfig;
use sqlx::FromRow;
use tokio::sync::RwLock;
use bcrypt::{hash,DEFAULT_COST};

#[derive(FromRow)]
struct ResponseModel{
    _user_name:String
}

pub struct RootCreditinals<'a>{
    pub config: &'a RwLock<AppConfig>

}

impl<'a> RootCreditinals<'a>{
    pub async fn init_db(&self,db:&SqlitePool){
        let root_uuid = uuid::Uuid::new_v4();
        let hashed_password = hash(&self.config.read().await.root_pass, DEFAULT_COST).unwrap();
        let data = sqlx::query_as::<_,ResponseModel>("SELECT user_name FROM users WHERE user_uuid='root'").
        fetch_one(db).await;
        if let Err(_err) = data{
            sqlx::query("INSERT INTO users (user_name,user_uuid,user_password,user_email) VALUES ('?','?','?','?');")
            .bind(self.config.read().await.root_name.to_string())
            .bind(root_uuid.to_string())
            .bind(hashed_password)
            .bind(self.config.read().await.root_email.to_string())
            .execute(db).await.unwrap();
        }
    }
}