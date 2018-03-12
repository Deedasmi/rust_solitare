extern crate board;

use board::Board;

fn main() {
    let b = Board::new();
    println!("{:?}", b);
    solve(b);
}

fn solve(mut b: Board) -> bool {
    if b.win() {
        return true
    }

    // Check for scored cards
    for i in 0..7 {
        let c = match b.cols[i].cards.last() {
            Some(x) => x,
            None => break,
        };
        if b.can_score(c) {
            if solve(b.score(i)) {
                return true
            }
        }
    }

    // Check for card moves

    // Check for drawn cards

    // Draw new cards

    // Fail
    println!("{:?}", b);
    return false
}