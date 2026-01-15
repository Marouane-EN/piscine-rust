use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let fildes = [&mut self.r, &mut self.g, &mut self.b, &mut self.a];
        let p1 = fildes.iter().position(|x| **x == first);
        let p2 = fildes.iter().position(|x| **x == second);
        if let (Some(x), Some(y)) = (p1, p2) {
            let a = *fildes[x];
            *fildes[x] = *fildes[y];
            *fildes[y] = a;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };

        assert_eq!(c.swap(c.r, c.b), Color { r: 10, g: 200, b: 255, a: 30 });
        assert_eq!(c.swap(c.r, c.g), Color { r: 200, g: 255, b: 10, a: 30 });
        assert_eq!(c.swap(c.r, c.a), Color { r: 30, g: 200, b: 10, a: 255 });

        assert_eq!(c.swap(c.g, c.r), Color { r: 200, g: 255, b: 10, a: 30 });
        assert_eq!(c.swap(c.g, c.b), Color { r: 255, g: 10, b: 200, a: 30 });
        assert_eq!(c.swap(c.g, c.a), Color { r: 255, g: 30, b: 10, a: 200 });

        assert_eq!(c.swap(c.b, c.r), Color { r: 10, g: 200, b: 255, a: 30 });
        assert_eq!(c.swap(c.b, c.g), Color { r: 255, g: 10, b: 200, a: 30 });
        assert_eq!(c.swap(c.b, c.a), Color { r: 255, g: 200, b: 30, a: 10 });

        assert_eq!(c.swap(c.a, c.r), Color { r: 30, g: 200, b: 10, a: 255 });
        assert_eq!(c.swap(c.a, c.b), Color { r: 255, g: 200, b: 30, a: 10 });
        assert_eq!(c.swap(c.a, c.g), Color { r: 255, g: 30, b: 10, a: 200 });
    }
}
