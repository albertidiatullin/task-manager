use dotenv::dotenv;
use std::env::{VarError};
use clap::Parser;
use configparser::ini::Ini;

#[derive(Parser)]
pub struct Settings{
    #[arg(long,env = "DATABASE_URL")]
    pub db_url:String
}


pub async fn load_env_file() -> Result<Settings,VarError>{
    dotenv().ok();

    let env_data = Settings::parse();
    Ok(env_data)

    

}


pub struct AppConfig{
    pub root_name:String,
    pub root_pass:String,
    pub root_email:String
}


impl AppConfig {
    pub fn build() -> Self{
    let mut config = Ini::new();
    config.load("root_cfg.cfg").expect("Config file not found");

    let name = config.get("root_data", "root_name")
        .expect("root_name is missing");
    let pass = config.get("root_data", "root_password")
        .expect("root_password is missing");
    let email = config.get("root_data", "root_email")
        .expect("root_email is missing");
    
    Self { 
        root_name: name, 
        root_pass: pass, 
        root_email: email 
    }

}}

