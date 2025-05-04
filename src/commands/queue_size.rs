use serenity::all::{CommandInteraction, CreateInteractionResponse};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, create_response, CommandHandler};

pub struct QueueSizeCommand;

impl QueueSizeCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for QueueSizeCommand {
    fn name(&self) -> &'static str {
        "kø"
    }

    fn description(&self) -> &'static str {
        "Se hvor mange som er foran deg i køen"
    }

    fn execute(
        &self,
        ctx: &WaffleContext,
        interaction: &CommandInteraction,
    ) -> CreateInteractionResponse {
        if !ctx.queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        let user_id = interaction.user.id.to_string();

        let message = match ctx.queue.index_of(user_id) {
            Some(index) => format!("Du er {} i køen", index + 1),
            None => "Du er ikke i køen".to_string(),
        };

        create_response(&message)
    }
}
