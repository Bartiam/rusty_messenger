use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// The port where the HTTP server will be running
    #[arg(long, env = "PORT", default_value_t = 3000)]
    pub port: u16,

    /// URL for connecting to the database
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    #[arg(long, env = "JWT_SECRET")]
    pub jwt_secret: String,

    #[arg(long, env = "REDIS_URL", default_value = "redis://127.0.0.1:6379")]
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self::parse()
    }
}
