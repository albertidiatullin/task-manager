use thiserror::Error;

#[derive(Debug,Error)]
pub enum CoreError{
    #[error("Ошибка загрузка env")]
    ErorrLoadEnv,
    #[error("Ошибка подключения к бд")]
    ErorrDBConnect
}