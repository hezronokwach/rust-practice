pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            // Returns the server URL or panics with no custom message
            server.unwrap().to_string()
        },
        Security::Message => {
            // Returns the server URL or panics with the message "ERROR: program stops"
            server.expect("ERROR: program stops").to_string()
        },
        Security::Warning => {
            // Returns the server URL or the message "WARNING: check the server"
            server.unwrap_or("WARNING: check the server").to_string()
        },
        Security::NotFound => {
            // Returns the server URL or the message "Not found: [MESSAGE]"
            match server {
                Ok(url) => url.to_string(),
                Err(msg) => format!("Not found: {}", msg),
            }
        },
        Security::UnexpectedUrl => {
            // Returns the error message or panics with the error message being the server URL
            match server {
                Ok(url) => panic!("{}", url),
                Err(msg) => msg.to_string(),
            }
        },
    }
}