use serenity::all::{CreateCommand, CreateInteractionResponse};

use super::create_ephemeral_response;

pub struct PingCommand;

impl PingCommand {
    pub fn run() -> CreateInteractionResponse {
        create_ephemeral_response("Pong!")
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("ping").description("Ping Vaffelbotten")
    }
}
