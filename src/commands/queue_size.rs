use std::sync::Arc;

use serenity::all::{CreateCommand, CreateInteractionResponse};

use crate::queue::WaffleQueue;

use super::{create_ephemeral_response, create_response};

pub struct QueueSizeCommand;

impl QueueSizeCommand {
    pub fn run(queue: Arc<WaffleQueue>, user_id: String) -> CreateInteractionResponse {
        if !queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        let message = match queue.index_of(user_id) {
            Some(index) => format!("Du er {} i køen", index + 1),
            None => "Du er ikke i køen".to_string(),
        };

        create_response(&message)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("kø").description("Se hvor mange som er foran deg i")
    }
}
