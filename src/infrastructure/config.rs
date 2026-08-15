use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// The port where the HTTP server will be running
    #[arg(long, env = "PORT", default_value_t = 3000)]
    pub port: u16,

    /// URL for connecting to the database
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self::parse()
    }
}