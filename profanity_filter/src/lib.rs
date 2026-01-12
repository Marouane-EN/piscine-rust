pub fn check_ms(message: &str) -> Result<&str, &str> {
    match message.contains("stupid") && !message.is_empty() {
      false => Ok(message),
      true => Err("ERROR: illegal")
    }
}