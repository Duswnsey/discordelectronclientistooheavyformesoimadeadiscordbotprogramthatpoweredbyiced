use iced::futures::{channel::mpsc, lock::Mutex};
use poise::serenity_prelude::Message;

pub struct State {
    pub msg_event_receiver: std::sync::Arc<Mutex<mpsc::Receiver<Message>>>,
    pub message_input:String
}
