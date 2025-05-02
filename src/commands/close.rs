use std::sync::Arc;

use serenity::all::{CreateCommand, CreateInteractionResponse};

use crate::queue::WaffleQueue;

use super::{create_ephemeral_response, create_response};

pub struct CloseCommand;

impl CloseCommand {
    pub fn run(bot: Arc<WaffleQueue>, is_oracle: bool) -> CreateInteractionResponse {
        if !is_oracle {
            return create_ephemeral_response("Kun orakler kan stenge for bestilling");
        }

        if !bot.is_open() {
            return create_ephemeral_response("Bestilling er allerede stengt");
        }

        bot.close();

        create_response("Bestilling er nå stengt")
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("stopp").description("Steng for bestilling av vafler")
    }
}
