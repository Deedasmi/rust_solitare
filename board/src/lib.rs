extern crate deck;

use deck::{Card, Deck};

pub trait Play<T> {
    fn play(&self, c: &Card) -> Result<T, ()>;
    fn can_play(&self, c: &Card) -> bool {
        self.play(c).is_ok()
    }
}

#[derive(Debug, Clone)]
pub struct Scored {
    cards: Vec<Card>,
}

impl Scored {
    pub fn new() -> Scored {
        Scored { cards: vec!() }
    }
}

impl Play<Scored> for Scored {
    fn play(&self, c: &Card) -> Result<Scored, ()> {
        if self.cards.len() == 0 {
            if c.rank == 1 {
                let mut n = self.cards.clone();
                n.push(c.clone());
                Ok(Scored { cards: n })
            } else {
                Err(())
            }
        } else {
            let m: &Card = self.cards.last().unwrap();
            if c.rank == m.rank + 1 && c.suit == m.suit {
                let mut n = self.cards.clone();
                n.push(c.clone());
                Ok(Scored { cards: n })
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Col {
    pub cards: Vec<Card>,
}

impl Col {
    pub fn new() -> Col {
        Col { cards: vec!() }
    }
    pub fn push(&mut self, c: Card) {
        self.cards.push(c);
    }
}

impl Play<Col> for Col {
    fn play(&self, c: &Card) -> Result<Col, ()> {
        if self.cards.len() == 0 {
            if c.rank == 14 {
                let mut n = self.cards.clone();
                n.push(c.clone());
                Ok(Col { cards: n })
            } else {
                Err(())
            }
        } else {
            let m = self.cards.last().unwrap();
            if c.rank == m.rank - 1 && c.color() != m.color() {
                let mut n = self.cards.clone();
                n.push(c.clone());
                Ok(Col { cards: n })
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    // Eigth col is drawn pile. 
    pub cols: [Col; 8],
    hearts: Scored,
    diamonds: Scored,
    spades: Scored,
    clubs: Scored,
    deck: Deck,
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {cols: [Col::new(), Col::new(), Col::new(), Col::new(), Col::new(), Col::new(), Col::new(), Col::new()], hearts: Scored::new(), diamonds: Scored::new(), spades: Scored::new(), clubs: Scored::new(), deck: Deck::new()};
        for i in 0..7 {
            for x in i..7 {
                b.cols[x].push(b.deck.draw().unwrap());
            }
        }
        b
    }
    pub fn can_score(&self, c: &Card) -> bool {
         match c.suit {
            ref Hearts => self.hearts.can_play(c),
            ref Diamonds => self.diamonds.can_play(c),
            ref Clubs => self.clubs.can_play(c),
            ref Spades => self.spades.can_play(c),
        }
    }
    pub fn score(&self, i: usize) -> Board {
        let mut b = self.clone();
        let c = b.cols[i].cards.pop().unwrap();
        match c.suit {
            Hearts => b.hearts = b.hearts.play(&c).unwrap(),
            Diamonds => b.diamonds = b.diamonds.play(&c).unwrap(),
            Clubs => b.clubs = b.clubs.play(&c).unwrap(),
            Spades => b.spades = b.spades.play(&c).unwrap(),
        }
        b
    }

    pub fn win(&self) -> bool {
        self.hearts.cards.len() == 13 && self.diamonds.cards.len() == 13 && self.clubs.cards.len() == 13 && self.spades.cards.len() == 13
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::{Card, Suit};
    #[test]
    fn play_col_good() {
        let c = Col { cards: vec!(Card { suit: Suit::Diamond, rank: 8 })};
        let a = Card { suit: Suit::Club, rank: 7 };
        c.play(a).unwrap();
    }
    #[test]
    #[should_panic]
    fn play_col_fail() {
let c = Col { cards: vec!(Card { suit: Suit::Diamond, rank: 8 })};
        let a = Card { suit: Suit::Club, rank: 8 };
        c.play(a).unwrap();
    }

    #[test]
    #[should_panic]
    fn play_col_fail2() {
let c = Col { cards: vec!(Card { suit: Suit::Diamond, rank: 8 })};
        let a = Card { suit: Suit::Heart, rank: 7 };
        c.play(a).unwrap();
    }
}
