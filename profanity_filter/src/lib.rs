
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

pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.to_lowercase().contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
