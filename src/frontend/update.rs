use crate::frontend::state::State;

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::MessageInput(content) => {
            state.message_input = content;
        }
        Message::Dummy => {}
    }
}
#[derive(Debug, Clone)]
pub enum Message {
    MessageInput(String),
    Dummy,
}
