use anyhow::Result;
use sqlx::{PgPool, Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions};

// Database optimization modules
pub mod config;

pub use config::DatabaseConfig;

/// Sets up the database connection with optimized pool configuration
/// Returns a ready-to-use database connection pool
pub async fn setup_database() -> Result<PgPool> {
    let config = DatabaseConfig::from_env();
    create_database_if_not_exists(&config.url).await?;
    let pool = create_optimized_connection_pool(&config).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

/// Creates an optimized connection pool with performance tuning
async fn create_optimized_connection_pool(config: &DatabaseConfig) -> Result<PgPool> {
    tracing::info!(
        "Creating optimized database pool: max_connections={}, min_connections={}, acquire_timeout={:?}",
        config.max_connections,
        config.min_connections,
        config.acquire_timeout
    );

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.acquire_timeout)
        .idle_timeout(Some(config.idle_timeout))
        .max_lifetime(Some(config.max_lifetime))
        .test_before_acquire(config.test_before_acquire)
        .connect(&config.url)
        .await?;
    
    tracing::info!("Database pool created successfully");
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

async fn run_migrations(pool: &PgPool) -> Result<()> {
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    tracing::info!("Migrations completed successfully");
    Ok(())
}
