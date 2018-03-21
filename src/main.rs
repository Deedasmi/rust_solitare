#[macro_use]
extern crate lazy_static;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::sync::Mutex;
extern crate board;
extern crate time;

use time::PreciseTime;
use board::Board;

use rusoto_core::Region;
use rusoto_core::EnvironmentProvider;
use rusoto_core::reactor::RequestDispatcher;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue};
use std::collections::HashMap;

lazy_static! {
    static ref SCORE: Mutex<Vec<u8>> = Mutex::new(vec!(0));
}

fn main() {
    let client = DynamoDbClient::new(RequestDispatcher::default(),EnvironmentProvider,Region::UsWest2);
    let b = Board::new();
    let mut items: HashMap<String, AttributeValue> = HashMap::new();
items.insert("deck".to_string(), AttributeValue { s: Some(b.get_id()), ..Default::default()});

    println!("{:?}", b);
    let s = PreciseTime::now();
    let r = solve(b, 0, false);
    let f = PreciseTime::now();
    match r {
        true => println!("Deck is solvable! Took {}", s.to(f)),
        false => println!("Deck not solvable :( Took {}", s.to(f)),
    }
    let score = SCORE.lock().unwrap().iter().max().unwrap().clone();
    SCORE.lock().unwrap().clear();
    items.insert("solveable".to_string(), AttributeValue {bool: Some(r), ..Default::default()});
    items.insert("score".to_string(), AttributeValue {n: Some(score.to_string()), ..Default::default()});
    items.insert("time".to_string(), AttributeValue {s: Some(format!("{}", s.to(f))), ..Default::default()});
    
    let input = PutItemInput {
        item: items,
        table_name: "games".to_string(),
        ..Default::default()
    };

    match client.put_item(&input).sync() {
        Ok(_) => println!("Success"),
        Err(x) => println!("{:?}", x)
    }
}

fn solve(b: Board, dep: u64, draw: bool) -> bool {
    if b.win() {
        SCORE.lock().unwrap().push(52);
        return true;
    }
    if b.scored() as u8 > *SCORE.lock().unwrap().iter().max().unwrap() {
        SCORE.lock().unwrap().push(b.scored() as u8);
        println!(
            "New high score: {}",
            *SCORE.lock().unwrap().iter().max().unwrap()
        );
    }

    let mut skip = draw;
    if skip {
        if let Some(x) = b.cols[7].cards.first() {
            skip = !b.can_score(x);
        }
        if skip {
            for d in 0..7 {
                if b.can_mov(7, d) {
                    skip = false;
                    break;
                }
            }
        }
    }

    if !skip {
        // Check for scored cards
        for i in 0..8 {
            let c = match b.cols[i].cards.last() {
                Some(x) => x,
                None => continue,
            };
            if b.can_score(c) {
                if solve(b.score(i), dep + 1, false) {
                    return true;
                }
            }
        }

        // Check for card moves
        for s in 0..8 {
            if b.cols[s].cards.first().is_none() {
                continue;
            };
            if b.cols[s].cards.first().expect("main").rank == 13 && b.cols[s].hidden.len() == 0
                && s != 7
            {
                continue;
            }
            for d in 0..7 {
                if b.can_mov(s, d) {
                    if solve(b.mov(s, d), dep + 1, false) {
                        return true;
                    }
                }
            }
        }
    }

    // Draw new cards
    if let Ok(x) = b.draw() {
        if solve(x, dep + 1, true) {
            return true;
        }
    }

    // Fail
    //println!("{:?}", b);
    return false;
}
