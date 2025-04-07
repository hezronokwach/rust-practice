
#[derive(Debug)]
pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Message {
        Self { content, user }
    }

    pub fn send_ms(&self) -> Option<String> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(self.content.clone())
        }
    }
}

pub fn check_ms(message: &Message) -> (bool, &str) {
    match message.send_ms() {
        None => (false, "ERROR: illegal"),
        Some(_) => (true, &message.content),
    }
}
