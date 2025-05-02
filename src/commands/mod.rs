use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};

pub mod bake;
pub mod close;
pub mod empty;
pub mod open;
pub mod ping;
pub mod queue_size;
pub mod waffle;

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
