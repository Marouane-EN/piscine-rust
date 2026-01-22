use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        Self(calc(value))
    }
}

fn calc(value: u32) -> Vec<RomanDigit> {
    let mut n = Vec::new();
    if value < 10 {
        match value {
            0 => n.push(RomanDigit::Nulla),
            1..=3 =>
                for _ in 1..=value {
                    n.push(RomanDigit::I);
                }
            4..=8 =>
                for i in 4..=value {
                    if value < 5 {
                        n.push(RomanDigit::I);
                        n.push(RomanDigit::V);
                    } else if value >= 5 && i == 4 {
                        n.push(RomanDigit::V);
                    } else if i < value {
                        n.push(RomanDigit::I);
                    }
                }
            9 => {
                n.push(RomanDigit::I);
                n.push(RomanDigit::X);
            }
            _ => panic!("dddd"),
        }
    } else if value < 40 {
        let m = value / 10;
        let u = value % 10;
        for _ in 1..=m {
            n.push(RomanDigit::X);
        }
        if u > 0 {

            n.append(&mut calc(u));
        }
    } else if value < 100 {
        let m = value / 10;
        let u = value % 10;
        for i in 4..=m {
            if m == 9 {
                n.push(RomanDigit::X);
                n.push(RomanDigit::C);
                break;
            }
            if m < 5 {
                n.push(RomanDigit::X);
                n.push(RomanDigit::L);
            } else if  m >= 5 && i == 4 {
                n.push(RomanDigit::L);
            } else if i < m{
                n.push(RomanDigit::X);
            }
        }
        if u > 0 {

            n.append(&mut calc(u));
        }
    } else if value < 400 {
        let m = value / 100;
        let u = value % 100;
        for _ in 1..=m {
            n.push(RomanDigit::C);
        }
        if u > 0 {

            n.append(&mut calc(u));
        }
    } else if value < 1000 {
        let m = value / 100;
        let u = value % 100;
        for i in 4..=m {
            if m == 9 {
                n.push(RomanDigit::C);
                n.push(RomanDigit::M);
                break;
            }
            if m < 5 {
                n.push(RomanDigit::C);
                n.push(RomanDigit::D);
            } else if m >= 5 && i == 4 {
                n.push(RomanDigit::D);
            } else if i < m{
                n.push(RomanDigit::C);
            }
        }
        if u > 0 {

            n.append(&mut calc(u));
        }
    } else if value < 4000 {
        let m = value / 1000;
        let u = value % 1000;
        for _ in 1..=m {
            n.push(RomanDigit::M);
        }
        if u > 0 {

            n.append(&mut calc(u));
        }
    }
    n
}
