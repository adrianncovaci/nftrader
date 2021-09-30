use super::message::Message;

#[derive(Default, Debug, Clone)]
pub struct Feed {
    messages: Vec<Message>
}

impl Feed {
    pub fn new() -> Self {
        Self {
            messages: vec![]
        }
    }

    pub fn get_messages(&self) -> std::slice::Iter<'_, Message> {
        self.messages.iter()
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
        self.messages.sort_by_key(|msg| msg.get_posted_date());
    }
}
