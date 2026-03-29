use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskDTO{
    pub task_name:String,
    pub task_value:String
}

#[derive(Deserialize)]
pub struct EditDTO{
    pub task_name:String,
    pub task_value:String
}