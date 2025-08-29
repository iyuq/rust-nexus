use anyhow::Result;
use sqlx::{PgPool, Postgres, migrate::MigrateDatabase};

/// Sets up the database connection, creates database if needed, and runs migrations
/// Returns a ready-to-use database connection pool
pub async fn setup_database(database_url: &str) -> Result<PgPool> {
    create_database_if_not_exists(database_url).await?;
    let pool = create_connection_pool(database_url).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

async fn create_database_if_not_exists(database_url: &str) -> Result<()> {
    if !Postgres::database_exists(database_url).await? {
        tracing::info!("Creating database...");
        Postgres::create_database(database_url).await?;
        tracing::info!("Database created successfully");
    }
    Ok(())
}

async fn create_connection_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

async fn run_migrations(pool: &PgPool) -> Result<()> {
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    tracing::info!("Migrations completed successfully");
    Ok(())
}
