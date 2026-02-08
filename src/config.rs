pub struct Config {
    pub redis_url: String,
    pub discord_token: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let redis_url = std::env::var("REDIS_URL").expect("Expected REDIS_URL in environment");
        let discord_token =
            std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

        Self {
            redis_url,
            discord_token,
        }
    }
}
