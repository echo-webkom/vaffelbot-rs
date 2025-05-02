use serenity::all::{CreateCommand, CreateInteractionResponse};

use crate::queue::WaffleQueue;

use super::{create_ephemeral_response, create_response};

pub struct WaffleCommand;

impl WaffleCommand {
    pub fn run(queue: &WaffleQueue, user_id: String) -> CreateInteractionResponse {
        if !queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        let message = match queue.index_of(user_id.clone()) {
            Some(index) => {
                format!("Du er {} i køen", index + 1)
            }
            None => {
                queue.push(user_id);
                let size = queue.size();
                format!("Du er nå i køen. Det er {} personer foran deg", size)
            }
        };

        create_response(&message)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("vaffel").description("Få en orakel til å steke vaffel til deg")
    }
}
