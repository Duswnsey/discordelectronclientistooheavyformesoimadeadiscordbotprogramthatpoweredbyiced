use poise::serenity_prelude::{self as serenity, EventHandler, Message};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: serenity::prelude::Context, msg: Message) {
        println!("{:#?}", msg.content);
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
        }
    }
}
