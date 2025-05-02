use serenity::{
    all::{
        CommandOptionType, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage, MessageBuilder, UserId,
    },
    builder::CreateCommand,
};

use crate::queue::WaffleQueue;

fn create_response(message: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(message))
}

fn create_ephemeral_response(message: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(message)
            .ephemeral(true),
    )
}

pub struct PingCommand;

impl PingCommand {
    pub fn run() -> CreateInteractionResponse {
        create_ephemeral_response("Pong!")
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("ping").description("Ping Vaffelbotten")
    }
}

pub struct QueueSizeCommand;

impl QueueSizeCommand {
    pub fn run(queue: &WaffleQueue, user_id: String) -> CreateInteractionResponse {
        if !queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        let message = match queue.index_of(user_id) {
            Some(index) => format!("Du er {} i køen", index),
            None => "Du er ikke i køen".to_string(),
        };

        create_response(&message)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("kø").description("Se hvor mange som er foran deg i")
    }
}

pub struct WaffleCommand;

impl WaffleCommand {
    pub fn run(queue: &WaffleQueue, user_id: String) -> CreateInteractionResponse {
        if !queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        let message = match queue.index_of(user_id.clone()) {
            Some(index) => format!("Du er {} i køen", index),
            None => {
                queue.push(user_id);
                format!(
                    "Du er nå i køen. Det er {} personer foran deg",
                    queue.size()
                )
            }
        };

        create_response(&message)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("vaffel").description("Få en orakel til å steke vaffel til deg")
    }
}

pub struct BakeCommand;

impl BakeCommand {
    pub fn run(
        queue: &WaffleQueue,
        amount: Option<i64>,
        is_oracle: bool,
    ) -> CreateInteractionResponse {
        if !queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        if !is_oracle {
            return create_ephemeral_response("Kun orakler kan steke vafler");
        }

        if amount.is_none() {
            return create_ephemeral_response("Du må spesifisere hvor mange vafler du vil steke");
        }

        let amount = amount.unwrap_or(1) as usize;

        let mut baked = 0;
        let mut baked_users = vec![];

        let n = queue.size().max(amount);

        for _ in 0..n {
            if let Some(user_id) = queue.pop() {
                baked += 1;
                baked_users.push(user_id);
            } else {
                break;
            }
        }

        let message = if baked == 0 {
            "Ingen vafler å steke".to_string()
        } else {
            let mut msg = MessageBuilder::new();
            msg.push("Stekte ").push(baked.to_string());

            if baked == 1 {
                msg.push(" en vaffel til: ");
                let user_id = UserId::new(baked_users[0].parse::<u64>().unwrap());
                msg.mention(&user_id);
            } else {
                msg.push(" vafler til: ");

                for (i, user_id) in baked_users.iter().enumerate() {
                    let user_id = UserId::new(user_id.parse::<u64>().unwrap());

                    if i == baked_users.len() - 1 {
                        msg.push(" og ").mention(&user_id);
                    } else {
                        msg.mention(&user_id).push(", ");
                    }
                }
            }

            msg.build()
        };

        create_response(&message)
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("stekt")
            .description("Stek vaffel")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "amount",
                    "Antall vafler å steke",
                )
                .required(true),
            )
    }
}

pub struct OpenCommand;

impl OpenCommand {
    pub fn run(bot: &WaffleQueue, is_oracle: bool) -> CreateInteractionResponse {
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

pub struct CloseCommand;

impl CloseCommand {
    pub fn run(bot: &WaffleQueue, is_oracle: bool) -> CreateInteractionResponse {
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
