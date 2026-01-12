pub fn check_ms(message: &str) -> Result<&str, &str> {
    match message.contains("stupid") {
      false => Ok(message),
      true => Err("ERROR: illegal")
    }
}