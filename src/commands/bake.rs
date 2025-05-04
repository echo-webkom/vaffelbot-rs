use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, MessageBuilder, UserId,
};

use crate::bot::WaffleContext;

use super::{create_ephemeral_response, create_response, CommandHandler};

pub struct BakeCommand;

impl BakeCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for BakeCommand {
    fn name(&self) -> &'static str {
        "stekt"
    }

    fn description(&self) -> &'static str {
        "Stek vaffel"
    }

    fn execute(
        &self,
        ctx: &WaffleContext,
        interaction: &CommandInteraction,
    ) -> CreateInteractionResponse {
        if !ctx.queue.is_open() {
            return create_ephemeral_response("Bestilling er stengt");
        }

        if !ctx.is_oracle {
            return create_ephemeral_response("Kun orakler kan steke vafler");
        }

        let amount = interaction.data.options.first().unwrap().value.as_i64();

        if amount.is_none() {
            return create_ephemeral_response("Du må spesifisere hvor mange vafler du vil steke");
        }

        let amount = amount.unwrap_or(1) as usize;

        let mut baked = vec![];
        let n = ctx.queue.size().min(amount);

        for _ in 0..n {
            if let Some(user_id) = ctx.queue.pop() {
                baked.push(user_id);
            } else {
                break;
            }
        }

        let message = if baked.is_empty() {
            "Ingen vafler å steke".to_string()
        } else {
            let mut msg = MessageBuilder::new();
            msg.push("Stekte ").push(baked.len().to_string());

            if baked.len() == 1 {
                msg.push(" en vaffel til: ");
                let user_id = UserId::new(baked[0].parse::<u64>().unwrap());
                msg.mention(&user_id);
            } else {
                msg.push(" vafler til: ");

                for (i, user_id) in baked.iter().enumerate() {
                    let user_id = UserId::new(user_id.parse::<u64>().unwrap());

                    if i == baked.len() - 1 {
                        msg.push(" og ").mention(&user_id);
                    } else {
                        msg.mention(&user_id).push(", ");
                    }
                }
            }

            if amount > n {
                msg.push(format!(" ({} vafler til overs)", amount - n));
            }

            msg.build()
        };

        create_response(&message)
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description(self.description())
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
