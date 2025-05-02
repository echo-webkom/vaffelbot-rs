use std::sync::Arc;

use serenity::all::{CreateCommand, CreateInteractionResponse};

use crate::queue::WaffleQueue;

use super::{create_ephemeral_response, create_response};

pub struct OpenCommand;

impl OpenCommand {
    pub fn run(bot: Arc<WaffleQueue>, is_oracle: bool) -> CreateInteractionResponse {
        if !is_oracle {
            return create_ephemeral_response("Kun orakler kan åpne for bestilling");
        }

        if bot.is_open() {
            return create_ephemeral_response("Bestilling er allerede åpnet");
        }

        bot.open();

        create_response("Bestilling er nå åpnet")
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("start").description("Åpne for bestilling av vafler")
    }
}
