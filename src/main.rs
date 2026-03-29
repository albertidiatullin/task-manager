use anyhow::Ok;
use axum::Router;
use task_manager::core::scurity::RootCreditinals;
use task_manager::core::settings::AppConfig;
use task_manager::databace::db_connect::db_connect;
use task_manager::databace::migrate::run_migrations;
use task_manager::core::state::get_state;
use task_manager::handlers::router::{return_router,need_auth_return_router};
use task_manager::middleware::logging_middleware::logging_middleware;
use axum::middleware::from_fn;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;


#[tokio::main]
async fn main() ->  anyhow::Result<()>{
    let state = get_state().await?;
    let db = db_connect().await?;
    run_migrations(&db).await.unwrap();
    let root_data = RootCreditinals{
        config:AppConfig::build()
    };
    root_data.init_db(&db).await;
    tracing_subscriber::fmt::init();
    let app = Router::new()
    .merge(return_router().await)
    .merge(need_auth_return_router().await?)
    .with_state(state)
    .layer(CookieManagerLayer::new())
    .layer(from_fn(logging_middleware));
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listen = TcpListener::bind(addr).await.unwrap();
    info!("⚡ SYSTEM BOOT → OK | CORE ONLINE | READY FOR ACTION ⚡");
    axum::serve(listen, app).await.unwrap();
    return Ok(());
}
