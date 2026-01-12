pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    let error_text = match server {
        Ok(text) => {
            return text.to_string();
        }
        Err(e) => e,
    };
    match security_level {
        Security::Unknown => panic!(),
        Security::Message => panic!("ERROR: program stops: {}", error_text),
        Security::Warning => panic!("WARNING: check the server"),
        Security::NotFound => panic!("Not found: {}", error_text),
        Security::UnexpectedUrl => panic!("{}", error_text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fetch_data(Err("server.com"), Security::Message), "server1.com");
    }
}
