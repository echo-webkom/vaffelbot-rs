use serenity::{
    all::{CommandInteraction, CreateInteractionResponse},
    async_trait,
};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, CommandHandler};

pub struct PingCommand;

impl PingCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandHandler for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Ping botten for å se om den er online"
    }

    fn execute(
        &self,
        _ctx: &WaffleContext,
        _interaction: &CommandInteraction,
    ) -> CreateInteractionResponse {
        create_ephemeral_response("Pong!")
    }
}
