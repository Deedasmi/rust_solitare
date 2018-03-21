extern crate deck;

use deck::{Card, Deck};
use deck::Suit::*;

const MAX_TURNS: u8 = 3;
const NUM_DRAW: usize = 3;

pub trait Play<T, U> {
    fn play(&self, c: &U) -> Result<T, ()>;
    fn can_play(&self, c: &U) -> bool {
        self.play(c).is_ok()
    }
}

#[derive(Debug, Clone)]
pub struct Scored {
    cards: Vec<Card>,
}

impl Scored {
    pub fn new() -> Scored {
        Scored { cards: vec![] }
    }
}

impl Play<Scored, Card> for Scored {
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
            let m: &Card = self.cards.last().expect("Scored play");
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
    pub hidden: Vec<Card>,
}

impl Col {
    pub fn new() -> Col {
        Col {
            cards: vec![],
            hidden: vec![],
        }
    }
    pub fn pop(&mut self) -> Option<Card> {
        let c = self.cards.pop();
        self.turn();
        c
    }
    pub fn turn(&mut self) {
        if self.cards.len() == 0 && self.hidden.len() > 0 {
            self.cards.push(self.hidden.pop().expect("Can't turn"));
        }
    }
}

impl Play<Col, Vec<Card>> for Col {
    fn play(&self, cards: &Vec<Card>) -> Result<Col, ()> {
        let c = cards.first().expect("Col play 1");
        if self.cards.len() == 0 {
            if c.rank == 13 {
                let mut n = self.cards.clone();
                n.append(cards.clone().as_mut());
                Ok(Col {
                    cards: n,
                    hidden: self.hidden.clone(),
                })
            } else {
                Err(())
            }
        } else {
            let m = self.cards.last().expect("Col Play 2");
            if c.rank == m.rank - 1 && c.color() != m.color() {
                let mut n = self.cards.clone();
                n.append(cards.clone().as_mut());
                Ok(Col {
                    cards: n,
                    hidden: self.hidden.clone(),
                })
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
    turns: u8,
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            cols: [
                Col::new(),
                Col::new(),
                Col::new(),
                Col::new(),
                Col::new(),
                Col::new(),
                Col::new(),
                Col::new(),
            ],
            hearts: Scored::new(),
            diamonds: Scored::new(),
            spades: Scored::new(),
            clubs: Scored::new(),
            deck: Deck::new(),
            turns: 0,
        };
        for i in 1..7 {
            b.cols[i - 1].cards.push(b.deck.draw().unwrap());
            for x in i..7 {
                b.cols[x].hidden.push(b.deck.draw().unwrap());
            }
        }
        b.cols[6].cards.push(b.deck.draw().unwrap());
        b
    }
    pub fn can_score(&self, c: &Card) -> bool {
        match c.suit {
            Heart => self.hearts.can_play(c),
            Diamond => self.diamonds.can_play(c),
            Club => self.clubs.can_play(c),
            Spade => self.spades.can_play(c),
        }
    }
    pub fn score(&self, i: usize) -> Board {
        let mut b = self.clone();
        let c = b.cols[i].pop().unwrap();
        match c.suit {
            Heart => b.hearts = b.hearts.play(&c).unwrap(),
            Diamond => b.diamonds = b.diamonds.play(&c).unwrap(),
            Club => b.clubs = b.clubs.play(&c).unwrap(),
            Spade => b.spades = b.spades.play(&c).unwrap(),
        }
        b
    }

    pub fn can_mov(&self, src: usize, dst: usize) -> bool {
        let r = self.cols[dst].can_play(&self.cols[src].cards);
        r
    }

    pub fn mov(&self, src: usize, dst: usize) -> Board {
        let mut b = self.clone();
        b.cols[dst] = b.cols[dst].play(&b.cols[src].cards).expect("Mov");
        b.cols[src].cards.clear();
        b.cols[src].turn();
        b
    }

    pub fn win(&self) -> bool {
        self.scored() == 52 as usize
    }
    pub fn scored(&self) -> usize {
        self.hearts.cards.len() + self.diamonds.cards.len() + self.clubs.cards.len()
            + self.spades.cards.len()
    }

    pub fn draw(&self) -> Result<Board, ()> {
        let mut b = self.clone();
        if b.deck.cards.len() == 0 {
            // Don't allow  if turns above max
            if b.turns == MAX_TURNS {
                return Err(());
            }

            // Reset deck
            b.cols[7]
                .hidden
                .push(b.cols[7].cards.pop().expect("Hidden"));
            b.deck = Deck::from(b.cols[7].hidden.clone());
            b.cols[7].hidden.clear();
            // increment
            b.turns += 1;
        }
        // Get 3 or remaining cards from deck
        let mut d = vec![];
        if b.deck.cards.len() < NUM_DRAW {
            d = b.deck.cards.clone();
            b.deck.cards.clear();
        } else {
            for _ in 0..NUM_DRAW {
                d.push(b.deck.cards.pop().expect("d push"));
            }
        }
        // Hide previous draw
        if b.cols[7].cards.len() > 0 {
            b.cols[7].hidden.push(b.cols[7].cards.pop().expect("hide"));
        }
        // Put on drawn pile
        b.cols[7].cards.push(d.pop().expect("put on"));
        b.cols[7].hidden.append(d.as_mut());

        Ok(b)
    }

    pub fn get_id(&self) -> String {
        format!("{}", self.deck)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deck::{Card, Suit};
    #[test]
    fn play_col_good() {
        let c = Col {
            cards: vec![
                Card {
                    suit: Suit::Diamond,
                    rank: 8,
                },
            ],
        };
        let a = Card {
            suit: Suit::Club,
            rank: 7,
        };
        c.play(a).unwrap();
    }
    #[test]
    #[should_panic]
    fn play_col_fail() {
        let c = Col {
            cards: vec![
                Card {
                    suit: Suit::Diamond,
                    rank: 8,
                },
            ],
        };
        let a = Card {
            suit: Suit::Club,
            rank: 8,
        };
        c.play(a).unwrap();
    }

    #[test]
    #[should_panic]
    fn play_col_fail2() {
        let c = Col {
            cards: vec![
                Card {
                    suit: Suit::Diamond,
                    rank: 8,
                },
            ],
        };
        let a = Card {
            suit: Suit::Heart,
            rank: 7,
        };
        c.play(a).unwrap();
    }
}
