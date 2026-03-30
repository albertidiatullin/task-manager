use anyhow::Ok;
use axum::{Router,middleware::from_fn};
use task_manager::core::{scurity::RootCreditinals,settings::AppConfig,state::get_state};
use task_manager::databace::{db_connect::db_connect,migrate::run_migrations};
use task_manager::handlers::router::{return_router,need_auth_return_router};
use task_manager::middleware::logging_middleware::logging_middleware;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;




#[tokio::main]
async fn main() ->  anyhow::Result<()>{
    let db = db_connect().await?;
    run_migrations(&db).await.unwrap();
    let load_config = AppConfig::build();
    let root_data = RootCreditinals{
        config:&load_config
        };
    root_data.init_db(&db).await;
    tracing_subscriber::fmt::init();

    // Connect to the main router //
    let app = Router::new()
    .merge(return_router().await)
    .merge(need_auth_return_router().await?)
    .with_state(get_state().await?)
    .layer(CookieManagerLayer::new())
    .layer(from_fn(logging_middleware));

    // Settings addr for axum serve //
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listen = TcpListener::bind(addr).await.unwrap();
    info!("⚡ SYSTEM BOOT → OK | CORE ONLINE | READY FOR ACTION ⚡");

    // Start axum serve //
    axum::serve(listen, app).await.unwrap();
    return Ok(());
}
