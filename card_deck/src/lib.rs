use rand::{ Rng };
#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let rng = rand::rng().random_range(1..=4);
        Suit::translate(rng)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            _ => Self::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let rng = rand::rng().random_range(1..=13);
        Rank::translate(rng)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.rank == Rank::Ace && card.suit == Suit::Spade {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };
        assert_eq!(winner_card(&your_card), true, "sqsdsqsds{:?}", your_card)
    }
}
