use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load variables from .env file
    dotenv().ok();

    // Using expect when the variable is required
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    println!("database_url: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;
    println!("connected database: {:?}", pool);
    Ok(())
}
