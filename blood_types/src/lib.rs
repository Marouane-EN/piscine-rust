use std::{ fmt, str::FromStr };

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_at(s.len() - 1);
        let a = match s1 {
            "A" => Antigen::A,
            "AB" => Antigen::AB,
            "B" => Antigen::B,
            "O" => Antigen::O,
            _ => {
                return Err(());
            }
        };
        let b = match s2 {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            _ => {
                return Err(());
            }
        };
        Ok(Self { antigen: a, rh_factor: b })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.antigen, self.rh_factor) {
            (Antigen::A, RhFactor::Negative) => write!(f, "A-"),
            (Antigen::A, RhFactor::Positive) => write!(f, "A+"),
            (Antigen::AB, RhFactor::Negative) => write!(f, "AB-"),
            (Antigen::AB, RhFactor::Positive) => write!(f, "AB+"),
            (Antigen::O, RhFactor::Negative) => write!(f, "O-"),
            (Antigen::O, RhFactor::Positive) => write!(f, "O+"),
            (Antigen::B, RhFactor::Negative) => write!(f, "B-"),
            (Antigen::B, RhFactor::Positive) => write!(f, "B+"),
        }
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let abo_ok = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::O || other.antigen == Antigen::A,
            Antigen::B => other.antigen == Antigen::O || other.antigen == Antigen::B,
            Antigen::AB => true,
        };

        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };

        abo_ok && rh_ok
    }

    pub fn donors(self) -> Vec<Self> {
        all_types()
            .into_iter()
            .filter(|blood| self.can_receive_from(*blood))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        all_types()
            .into_iter()
            .filter(|blood| blood.can_receive_from(self))
            .collect()
    }
}

fn all_types() -> Vec<BloodType> {
    vec![
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        }
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
