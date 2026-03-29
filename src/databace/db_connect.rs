use sqlx::{SqlitePool};
use crate::core::settings::load_env_file;
use crate::exceptions::core_exception::CoreError;


pub async fn db_connect() -> Result<SqlitePool,CoreError>{
    let db_url = load_env_file().await.map_err(|_e| CoreError::ErorrLoadEnv)?.db_url;
    let connect=SqlitePool::connect(&db_url).await.map_err(|_e| CoreError::ErorrDBConnect)?;
    Ok(connect)
}