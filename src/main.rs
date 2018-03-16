extern crate board;
extern crate time;

use time::PreciseTime;
use board::Board;

fn main() {
    let b = Board::new();
    println!("{:?}", b);
    let s = PreciseTime::now();
    let r = solve(b);
    let f = PreciseTime::now();
    match r {
        true => println!("Deck is solvable! Took {}", s.to(f)),
        false => println!("Deck not solvable :( Took {}", s.to(f)),
    }
}

fn solve(mut b: Board) -> bool {
    if b.win() {
        return true;
    }

    // Check for scored cards
    for i in 0..7 {
        let c = match b.cols[i].cards.last() {
            Some(x) => x,
            None => continue,
        };
        if b.can_score(c) {
            if solve(b.score(i)) {
                return true;
            }
        }
    }

    // TODO track which cards are 'face down'.
    // Check for card moves
    for s in 0..8 {
        let c = match b.cols[s].cards.last() {
            Some(x) => x,
            None => continue,
        };
        for d in 0..7 {}
    }

    // Check for drawn cards

    // Draw new cards

    // Fail
    println!("{:?}", b);
    return false;
}
