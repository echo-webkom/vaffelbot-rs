pub struct Config {
    pub redis_url: String,
    pub discord_token: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let redis_url = std::env::var("REDIS_URL").expect("Expected REDIS_URL in environment");
        let discord_token =
            std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let database_url =
            std::env::var("DATABASE_URL").expect("Expected DATABASE_URL in environment");

        Self {
            redis_url,
            discord_token,
            database_url,
        }
    }
}
