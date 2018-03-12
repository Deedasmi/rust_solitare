#[macro_use]
extern crate itertools;
extern crate rand;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    pub fn all() -> Vec<Suit> {
        use Suit::*;
        vec![Spade, Heart, Diamond, Club]
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{:?}", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(suit: Suit, rank: u8) -> Card {
        Card { suit, rank }
    }

    pub fn color(&self) -> Color {
        use Color::*;
        use Suit::*;
        match self.suit {
            Diamond => Red,
            Heart => Red,
            Spade => Black,
            Club => Black,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut d: Vec<Card> = Vec::new();
        let ranks = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14];
        for (s, r) in iproduct!(Suit::all(), ranks) {
            d.push(Card::new(s, r));
        }
        let mut sd = Deck { cards: d };
        sd.shuffle();
        sd
    }
    pub fn shuffle(&mut self) {
        let mut r = rand::OsRng::new().expect("Couldn't get RNG");
        let mut c = self.cards.clone();
        let c = c.as_mut_slice();
        r.shuffle::<Card>(c);
        self.cards = c.to_vec();
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_cards() {
        let d = Deck::new();
        assert_eq!(d.cards.len(), 52);
    }
}
