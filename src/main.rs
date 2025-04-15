use dotenv::dotenv;
use slac::{router::get_router, state::AppState};
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load variables from .env file
    dotenv().ok();

    // Using expect when the variable is required
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    println!("database_url: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;
    println!("connected database: {:?}", pool);

    let state = AppState::new(pool);
    let router = get_router(state).await?;

    let addr = format!("0.0.0.0:{}", "6869");
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
