use std::time::Duration;

/// Database configuration for optimal performance
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub test_before_acquire: bool,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        // Parse environment variables with sensible defaults for high load
        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "100".to_string())  // Increased from 50 to 100
            .parse()
            .unwrap_or(100);
            
        let min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())   // Increased from 5 to 10
            .parse()
            .unwrap_or(10);
            
        let acquire_timeout_secs = std::env::var("DB_ACQUIRE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "5".to_string())    // Increased from 3 to 5 seconds
            .parse()
            .unwrap_or(5);
            
        let idle_timeout_secs = std::env::var("DB_IDLE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "600".to_string())
            .parse()
            .unwrap_or(600);
            
        let max_lifetime_secs = std::env::var("DB_MAX_LIFETIME_SECS")
            .unwrap_or_else(|_| "1800".to_string())
            .parse()
            .unwrap_or(1800);

        Self {
            url,
            max_connections,
            min_connections,
            acquire_timeout: Duration::from_secs(acquire_timeout_secs),
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            max_lifetime: Duration::from_secs(max_lifetime_secs),
            test_before_acquire: true,
        }
    }
}
