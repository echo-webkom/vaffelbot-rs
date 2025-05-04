use serenity::all::{ActivityData, CommandInteraction, CreateInteractionResponse, OnlineStatus};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, create_response, CommandHandler};

pub struct OpenCommand;

impl OpenCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for OpenCommand {
    fn name(&self) -> &'static str {
        "start"
    }

    fn description(&self) -> &'static str {
        "Åpne for bestilling av vafler"
    }

    fn execute(
        &self,
        ctx: &WaffleContext,
        _interaction: &CommandInteraction,
    ) -> CreateInteractionResponse {
        if !ctx.is_oracle {
            return create_ephemeral_response("Kun orakler kan åpne for bestilling");
        }

        if ctx.queue.is_open() {
            return create_ephemeral_response("Bestilling er allerede åpnet");
        }

        ctx.queue.open();

        ctx.context.set_presence(
            Some(ActivityData::playing("🧇 Lager vafler")),
            OnlineStatus::Online,
        );

        create_response("Bestilling er nå åpnet")
    }
}
