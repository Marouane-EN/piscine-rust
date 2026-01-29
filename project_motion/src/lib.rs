#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        Self {
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            init_position,
            init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.0;
        let g: f32 = 9.8;
        let t: f32 = self.time;
          let vx = self.init_velocity.x;
        let vy = self.init_velocity.y - g * t;

        let x = self.init_position.x + self.init_velocity.x * t;
        let y = self.init_position.y + self.init_velocity.y * t - 0.5 * g * t * t;

        self.actual_velocity = Object {
            x: r(vx),
            y: r(vy),
        };
        self.actual_position = Object {
            x: r(x),
            y: r(y),
        };

        if self.actual_position.y <= 0.0 {
            return None;
        }

        Some(self.clone())
    }
}
fn r(x: f32) -> f32 {
    (x * 10.0).round() / 10.0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    }
}
