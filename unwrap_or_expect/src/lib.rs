pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown =>
            match server {
                Ok(text) => text.to_string(),
                Err(_) => panic!(),
            }
        Security::Message =>
            match server {
                Ok(text) => text.to_string(),

                Err(_) => panic!("ERROR: program stops"),
            }
        Security::Warning =>
            match server {
                Ok(text) => text.to_string(),

                Err(_) => panic!("WARNING: check the server"),
            }
        Security::NotFound =>
            match server {
                Ok(text) => text.to_string(),

                Err(text) => panic!("Not found: {}", text),
            }
        Security::UnexpectedUrl =>
            match server {
                Ok(text) => text.to_string(),

                Err(text) => panic!("{}", text),
            }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fetch_data(Ok("server1.com"), Security::Warning),"server1.com");
    }
}
