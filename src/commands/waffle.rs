use serenity::all::{CommandInteraction, CreateInteractionResponse};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, create_response, CommandHandler};

pub struct WaffleCommand;

impl WaffleCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for WaffleCommand {
    fn name(&self) -> &'static str {
        "vaffel"
    }

    fn description(&self) -> &'static str {
        "Få en orakel til å steke vaffel til deg"
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

        let message = match ctx.queue.index_of(user_id.clone()) {
            Some(index) => {
                format!("Du er {} i køen", index + 1)
            }
            None => {
                let size = ctx.queue.size();
                ctx.queue.push(user_id);
                format!("Du er nå i køen. Det er {} personer foran deg", size)
            }
        };

        create_response(&message)
    }
}
