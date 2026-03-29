
use sqlx::sqlite::SqlitePool;





pub async  fn run_migrations(db:&SqlitePool) -> Result<(),sqlx::Error>{

    
    sqlx::migrate!("src/migrations")
    .run(db)
    .await?;



    Ok(())
    
}