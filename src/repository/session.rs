
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqlitePool;
use sqlx::{self, query, query_as};
use  chrono::Utc;


#[derive(Clone)]
pub struct SessionRepository{
    pub db_pool:SqlitePool
}
#[derive(FromRow)]
pub struct SessionDomailModel{
    pub user_uuid:String,
    pub session_experation:String
}


impl SessionRepository {
    pub async fn get_session(&self,user_name:&String) -> Result<Option<SessionDomailModel>,sqlx::Error>{
        let session = query_as::<_,SessionDomailModel>("SELECT user_uuid,session_experation FROM session WHERE user_name = ?")
        .bind(user_name.to_string())
        .fetch_optional(&self.db_pool).await?;
        
        Ok(session)
    }
    pub async fn add_session(&self,session_id:String,user_name:&String) -> Result<(),sqlx::Error>{
        let time = Utc::now() + chrono::Duration::hours(1);
        query("INSERT INTO session (user_uuid,user_name,session_experation) VALUES (?,?,?)")
        .bind(session_id)
        .bind(user_name.to_string())
        .bind(time.to_string())
        .execute(&self.db_pool).await?;

        Ok(())
    }

    pub async fn delete_session(&self,user_uuid:String)-> Result<bool,sqlx::Error>{
        query("DELETE FROM session WHERE user_uuid = ?")
        .bind(user_uuid.to_string())
        .execute(&self.db_pool).await?;

        Ok(true)
      }
}