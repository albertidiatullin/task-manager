use chrono::Utc;
use serde::Serialize;
use sqlx::{prelude::FromRow, sqlite::SqlitePool};

use crate::dto::task::{CreateTaskDTO,EditDTO};

#[derive(Clone)]
pub struct TaskRepository{
    pub sqlite_pool:SqlitePool
}

#[derive(FromRow)]
pub struct Tasks{
    pub id: i32,
    pub task_name: String,
    pub task_value: String,
    pub user_uuid: String,
    pub user_name: String,
    pub task_data: String
}

#[derive(Serialize)]
pub struct TaskModelDomain{
    pub task_name: String,
    pub task_value: String,
    pub task_data: String
}



impl From<Tasks> for TaskModelDomain {
    fn from(value: Tasks) -> Self {
        Self { task_name:value.task_name, 
            task_value: value.task_value,
             task_data: value.task_data }
    }
}




impl TaskRepository {
    pub async fn add_task(&self,task_data:&CreateTaskDTO,
                        user_uuid:&String,
                        user_name:&String) -> Result<bool,sqlx::Error>{
        let time = Utc::now();
        sqlx::query("INSERT INTO tasks(task_name,task_value,user_uuid,user_name,task_data) VALUES (?,?,?,?,?)")
        .bind(&task_data.task_name)
        .bind(&task_data.task_value)
        .bind(user_uuid)
        .bind(user_name)
        .bind(time.to_string())
        .execute(&self.sqlite_pool).await?;
        
        Ok(true)

    }

    pub async fn edit_task<'a>(&self,user_name:&'a str,edit_data:&EditDTO) -> Result<bool,sqlx::Error>{
        sqlx::query("UPDATE tasks SET task_name=?,task_value=? WHERE user_name=?")
        .bind(&edit_data.task_name)
        .bind(&edit_data.task_value)
        .bind(user_name).execute(&self.sqlite_pool).await?;
        
        Ok(true)
    }

    pub async fn get_all_tasks(&self,
                                user_name:&String) -> Result<Vec<Tasks>,sqlx::Error>{
        let data = sqlx::query_as::<_,Tasks>("SELECT * FROM tasks WHERE user_name=?")
        .bind(user_name.to_string())
        .fetch_all(&self.sqlite_pool).await?;
        Ok(data)
        }
    
    pub async fn get_task_by_id<'a>(&self,
                                user_name:&'a str,
                                task_id:&'a str) -> Result<TaskModelDomain,sqlx::Error>{
        let data = sqlx::query_as  ::<_,Tasks>("SELECT * FROM tasks WHERE id=?,user_name=?")
        .bind(task_id)
        .bind(user_name)
        .fetch_one(&self.sqlite_pool).await?;
        let return_data:TaskModelDomain = data.into();
        Ok(return_data)


        }
    
    pub async fn delete_task_in_db(&self,task_id:String) -> Result<bool,sqlx::Error>{
        sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(task_id)
        .execute(&self.sqlite_pool).await?;
        Ok(true)
    }

        

}