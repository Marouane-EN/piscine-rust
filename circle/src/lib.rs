use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);
impl Point {
    pub fn distance(&self, point: Point) -> f64 {
        let sum = (&self.0 - point.0).powi(2) + (&self.1 - point.1).powi(2);
        sum.sqrt()
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(c_x: f64, c_y: f64, r: f64) -> Self {
        Self { center: Point(c_x, c_y), radius: r }
    }
    pub fn diameter(&self) -> i32 {
        (self.radius * 2.0) as i32
    }
    pub fn area(&self) -> f64 {
        self.radius.powi(2) * PI
    }
    pub fn intersect(&self, circle: Circle) -> bool {
        let d = self.center.distance(circle.center);
        if self.radius + circle.radius > d {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };
        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);
        assert_eq!(circle.area(), 70685.83470577035);
        assert_eq!(circle.diameter(), 300);

        assert_eq!(circle1.diameter(), 60);
        assert_eq!(circle.intersect(circle1), false);

        assert_eq!(point_a.distance(point_b), 1.4142135623730951);
    }
}
