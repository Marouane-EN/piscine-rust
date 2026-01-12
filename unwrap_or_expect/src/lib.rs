pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),

        Security::Message => server.expect("ERROR: program stops").to_string(),

        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),

        Security::NotFound =>
            match server {
                Ok(text) => text.to_string(),
                Err(text) => format!("Not found: {}", text),
            }

        Security::UnexpectedUrl => server.unwrap_err().to_string(),
    }
}
