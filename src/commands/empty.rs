use serenity::all::{CommandInteraction, CreateInteractionResponse, MessageBuilder, UserId};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, create_response, CommandHandler};

pub struct EmptyCommand;

impl EmptyCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for EmptyCommand {
    fn name(&self) -> &'static str {
        "tøm"
    }

    fn description(&self) -> &'static str {
        "Tøm køen"
    }

    fn execute(
        &self,
        ctx: &WaffleContext,
        _interation: &CommandInteraction,
    ) -> CreateInteractionResponse {
        if !ctx.is_oracle {
            return create_ephemeral_response("Kun orakler kan tømme køen");
        }

        let users = ctx.queue.drain();
        ctx.queue.close();

        if users.is_empty() {
            return create_response("Køen er allerede tom");
        }

        if users.len() == 1 {
            let user_id = UserId::new(users[0].parse::<u64>().unwrap());
            return create_response(&format!("Tømte køen. {} ble fjernet", user_id));
        }

        let mut msg = MessageBuilder::new();
        msg.push("Tømte køen. Fjernet: ");
        for (i, user_id) in users.iter().enumerate() {
            let user_id = UserId::new(user_id.parse::<u64>().unwrap());

            if i == users.len() - 1 {
                msg.push(" og ").mention(&user_id);
            } else {
                msg.mention(&user_id).push(", ");
            }
        }
        msg.push(" fra køen");

        let message = msg.build();

        create_response(&message)
    }
}
