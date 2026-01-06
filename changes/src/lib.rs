#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self { alias: alias.to_string(), brightness: 0 }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights {
        if light.alias == alias {
            light.brightness = value;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);
        assert_eq!(lights[0].brightness, 0);
        change_brightness(&mut lights, "living_room", 200);
        assert_eq!(lights[0].brightness, 200);

    }
}
