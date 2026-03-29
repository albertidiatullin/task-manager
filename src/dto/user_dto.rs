use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone)]
pub struct RegisterDTO{
    pub user_name:String,
    pub user_password:String,
    pub user_email:String
    
}


#[derive(Deserialize)]
pub struct AuthDTO{
    pub user_name:String,
    pub user_password:String
}