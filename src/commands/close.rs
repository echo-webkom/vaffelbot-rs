use serenity::all::{CommandInteraction, CreateInteractionResponse, OnlineStatus};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, create_response, CommandHandler};

pub struct CloseCommand;

impl CloseCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for CloseCommand {
    fn name(&self) -> &'static str {
        "stopp"
    }

    fn description(&self) -> &'static str {
        "Steng for bestilling av vafler"
    }

    fn execute(
        &self,
        ctx: &WaffleContext,
        _interaction: &CommandInteraction,
    ) -> CreateInteractionResponse {
        if !ctx.is_oracle {
            return create_ephemeral_response("Kun orakler kan stenge for bestilling");
        }

        if !ctx.queue.is_open() {
            return create_ephemeral_response("Bestilling er allerede stengt");
        }

        ctx.queue.close();

        ctx.context.set_presence(None, OnlineStatus::DoNotDisturb);

        create_response("Bestilling er nå stengt")
    }
}
