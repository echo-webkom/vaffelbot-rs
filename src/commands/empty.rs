use serenity::all::{CreateCommand, CreateInteractionResponse, MessageBuilder, UserId};

use crate::queue::WaffleQueue;

use super::{create_ephemeral_response, create_response};

pub struct EmptyCommand;

impl EmptyCommand {
    pub fn run(bot: &WaffleQueue, is_oracle: bool) -> CreateInteractionResponse {
        if !is_oracle {
            return create_ephemeral_response("Kun orakler kan tømme køen");
        }

        let users = bot.drain();
        bot.close();

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

    pub fn register() -> CreateCommand {
        CreateCommand::new("tøm").description("Tøm køen")
    }
}
