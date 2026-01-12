pub fn check_ms(message: &str) -> Result<&str, &str> {
    match message.contains("stupid") {
      true => Ok(message),
      false => Err("ERROR: illegal")
    }
}