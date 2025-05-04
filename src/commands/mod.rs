use serenity::{
    all::{
        CommandInteraction, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    async_trait,
};

use crate::bot::WaffleContext;

pub mod bake;
pub mod close;
pub mod empty;
pub mod open;
pub mod ping;
pub mod queue_size;
pub mod waffle;

#[async_trait]
pub trait CommandHandler: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(
        &self,
        ctx: &WaffleContext,
        interaction: &CommandInteraction,
    ) -> CreateInteractionResponse;
    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description(self.description())
    }
}

pub fn create_response(message: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(message))
}

pub fn create_ephemeral_response(message: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(message)
            .ephemeral(true),
    )
}
