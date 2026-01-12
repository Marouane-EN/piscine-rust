use std::{ collections::HashMap, fmt::{ Error, format }, num::ParseFloatError };

pub struct Flag {
    short_hand: String,

    long_hand: String,

    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().nth(0).unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match self.flags.get(input).unwrap()(argv[0], argv[1]) {
            Ok(v) => Ok(v),
            Err(r) => Err(r.to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x = a.parse::<f64>()?;
    let y = b.parse::<f64>()?;
    Ok((x / y).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x = a.parse::<f64>()?;
    let y = b.parse::<f64>()?;
    Ok((x % y).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
