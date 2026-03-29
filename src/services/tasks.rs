
use crate::{dto::task::{CreateTaskDTO, EditDTO}, exceptions::exception::AppError, 
    repository::{tasks::{TaskModelDomain, TaskRepository}, user::{DataForHandler, UserRepository}}};


#[derive(Clone)]
pub struct TaskService{
    pub task_repo:TaskRepository,
    pub user_repo:UserRepository
    
}



impl TaskService{
    pub async fn add_task_data<'a>(&self,task_data:&CreateTaskDTO,user_data:&DataForHandler) -> Result<bool,AppError>{
    let map_err = self.task_repo.add_task(task_data, &user_data.user_uuid, &user_data.user_name).
    await.map_err(|_e| AppError::DBErorr)?;
    let data = map_err;
    Ok(data)
    }

    pub async fn create_task<'a>(&self,payload:CreateTaskDTO,get_uuid_cookie:&'a str) -> Result<(),AppError>{
        let get_uuid_model = self.user_repo.convert(get_uuid_cookie).await.map_err(|_e|AppError::UserNotFound)?;
        let user = self.user_repo.get_user_for_create_task(get_uuid_model).await.ok_or(AppError::UserNotFound)?;
        self.add_task_data(&payload, &user).await?;
        Ok(())
    }
    
    pub async fn get_all_tasks<'a>(&self,get_uuid_cookie:&'a str) -> Result<Vec<TaskModelDomain>,AppError>{
        let model = self.user_repo.convert(get_uuid_cookie).await.map_err(|e| AppError::InternalServerError { err: e.to_string() })?;
        let user_data = self.user_repo.get_user_of_uuid(model).await.map_err(|_e| AppError::UserNotFound)?;
        let tasks= self.task_repo.get_all_tasks(&user_data.user_name).await.
        map_err(|_e|AppError::TaskNotFound)?;
        let converted_task:Vec<TaskModelDomain> = tasks.into_iter()
        .map(TaskModelDomain::from).collect();
        Ok(converted_task)
    }


    pub async fn get_task_by_id<'a,'b>(&self,get_uuid_cookie:&'a str,task_id:&'b str) -> Result<TaskModelDomain,AppError>{
        let model = self.user_repo.convert(get_uuid_cookie).await.map_err(|e| 
            AppError::InternalServerError { err: e.to_string() })?;
        let user = self.user_repo.get_user_of_uuid(model).await.map_err(|_e| AppError::UserNotFound)?;
        let task = self.task_repo.
        get_task_by_id(&user.user_name,task_id).await
        .map_err(|_e| AppError::TaskNotFound)?; 
        Ok(task)
    }

    
    pub async fn edit_task<'a>(&self,get_uuid_cookie:&'a str,edit_data:&EditDTO) -> Result<bool,AppError>{
            let model = self.user_repo.convert(get_uuid_cookie).await.map_err(|e| AppError::InternalServerError { err: e.to_string() })?;
            let get_user = self.user_repo.get_user_of_uuid(model).await.map_err(|_e|AppError::UserNotFound)?; 
            let edit_task = self.task_repo.edit_task(&get_user.user_name, edit_data).
            await.map_err(|_e| AppError::DBErorr)?;
            Ok(edit_task)
    }

    pub async fn delete_task<'b,'a>(&self,task_id:&'b str,get_uuid_cookie:&'a str) -> Result<bool,AppError>{
        let model = self.user_repo.convert(get_uuid_cookie).await
        .map_err(|e| AppError::InternalServerError { err: e.to_string() })?;
        self.user_repo.get_user_of_uuid(model).await.map_err(|_e|AppError::UserNotFound)?; 
        self.task_repo.delete_task_in_db(task_id.to_string()).
        await.map_err(|_e| AppError::DBErorr)?;
        Ok(true)
    }
}

