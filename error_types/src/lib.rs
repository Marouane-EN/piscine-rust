use chrono::Utc;
// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%F %T").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }
        if self.password.len() < 8 {
            return Err(
                FormError::new(
                    "password",
                    self.password.clone(),
                    "Password should be at least 8 characters long"
                )
            );
        }
        let mut valid = [false, false, false];
        for c in self.password.chars() {
            if !c.is_alphanumeric() && c.is_ascii() && !valid[0] {
                valid[0] = true;
            }
            if c.is_numeric() && !valid[1] {
                valid[1] = true;
            }
            if c.is_alphabetic() && !valid[2] {
                valid[2] = true;
            }
        }
        if valid.contains(&false) {
            return Err(
                FormError::new(
                    "password",
                    self.password.clone(),
                    "Password should be a combination of ASCII numbers, letters and symbols"
                )
            );
        }
        Ok(())
    }
}

// Integration tests live under tests/; keep this file focused on the public API.
