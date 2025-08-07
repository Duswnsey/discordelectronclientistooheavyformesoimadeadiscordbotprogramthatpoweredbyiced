use dotenv::dotenv;
use poise::serenity_prelude as serenity;
mod japda;
use crate::frontend::{index::Handler, state::State, update::update, views::view};
mod frontend;
use iced::futures::channel::mpsc;
pub struct Data {
    msg_event_sender: mpsc::Sender<poise::serenity_prelude::Message>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// why
#[poise::command(slash_command)]
async fn a(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(String::from("이거 왜함?")).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> iced::Result {
    dotenv().ok();
    let (sender, reciver) = mpsc::channel(100);
    let _thread = std::thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
            let intents = serenity::GatewayIntents::all();
            let framework = poise::Framework::builder()
                .options(poise::FrameworkOptions {
                    commands: vec![a()],
                    ..Default::default()
                })
                .setup(|ctx, _ready, framework| {
                    Box::pin(async move {
                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?;
                        Ok(Data {
                            msg_event_sender: sender,
                        })
                    })
                })
                .build();

            let client = serenity::ClientBuilder::new(token, intents)
                .activity(serenity::ActivityData::playing(String::from("사망")))
                .framework(framework)
                .event_handler(Handler)
                .await;

            client.unwrap().start().await.unwrap();
        })
    });
    let receiverlock = std::sync::Arc::new(iced::futures::lock::Mutex::new(reciver));
    iced::application(
        move || State {
            msg_event_receiver: receiverlock.clone(),
            message_input:String::new()
        },
        update,
        view,
    )
    .title("An iced app")
    .run()
}
