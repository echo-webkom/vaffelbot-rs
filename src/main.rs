mod bot;
mod commands;
mod queue;

use std::env;

use bot::WaffleBot;
use dotenv::dotenv;
use queue::WaffleQueue;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("Expected REDIS_URL in environment");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let guild_id = env::var("GUILD_ID")
        .expect("Expected GUILD_ID in environment")
        .parse()
        .expect("GUILD_ID must be an integer");

    let redis = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let waffle_queue = WaffleQueue::new(redis);
    let waffle_bot = WaffleBot::new(token, guild_id, waffle_queue);

    if let Err(why) = waffle_bot.start().await {
        println!("Client error: {why:?}");
    }
}
