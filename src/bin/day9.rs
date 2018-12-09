// day9.rs
// marble game

use lazy_static::*;
use regex::Regex;
use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let (player_count, last_marble_value) = parse_line(&input);
    println!(
        "part 1: {}",
        find_highest_score(player_count, last_marble_value)
    );
    println!(
        "part 2: {}",
        find_highest_score(player_count, 100 * last_marble_value)
    );
}

// part 1 and 2
// (number of players, last marble value)
type GameSetup = (usize, u32);

struct GameState {
    marbles: VecDeque<u32>,
    current_val: u32,
}

impl GameState {
    fn new() -> GameState {
        let mut marbles = VecDeque::new();
        marbles.push_back(0);
        GameState {
            marbles,
            current_val: 0,
        }
    }

    // place a marble, returning any addition to score
    fn place_marble(&mut self) -> u32 {
        // Convention is that the current marble is at the front
        self.current_val += 1;
        if self.current_val % 23 == 0 {
            // rotate deque backwards 6 times then remove the 7th marble
            for _ in 0..6 {
                let x = self.marbles.pop_back().unwrap();
                self.marbles.push_front(x);
            }
            self.current_val + self.marbles.pop_back().unwrap()
        } else {
            // rotate deque forward twice then add marble
            for _ in 0..2 {
                let x = self.marbles.pop_front().unwrap();
                self.marbles.push_back(x);
            }
            self.marbles.push_front(self.current_val);
            0
        }
    }
}

fn find_highest_score(player_count: usize, last_marble_value: u32) -> u32 {
    let mut state = GameState::new();
    state.marbles.reserve(last_marble_value as usize);
    let mut scores = vec![0; player_count];
    let mut current_player = 0;

    for _ in 0..last_marble_value {
        current_player = (current_player + 1) % player_count;
        scores[current_player] += state.place_marble();
    }

    *scores.iter().max().unwrap()
}

fn parse_line(line: &str) -> GameSetup {
    lazy_static! {
        static ref RE_INPUT: Regex =
            Regex::new(r"^(\d+) players; last marble is worth (\d+)").unwrap();
    }
    if let Some(caps) = RE_INPUT.captures(line) {
        let player_count: usize = caps[1].parse().unwrap();
        let last_marble_value: u32 = caps[2].parse().unwrap();
        (player_count, last_marble_value)
    } else {
        panic!("Invalid line!")
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(
        (10, 1618),
        parse_line("10 players; last marble is worth 1618 points")
    );
}

#[test]
fn test_place_marble() {
    let mut state = GameState::new();
    for _ in 0..25 {
        state.place_marble();
    }
    assert_eq!(
        vec![
            25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 19, 2, 24, 20,
        ],
        Vec::from(state.marbles)
    );
}

#[test]
fn test_find_highest_score() {
    assert_eq!(0, find_highest_score(9, 22));
    assert_eq!(32, find_highest_score(9, 23));
    assert_eq!(32, find_highest_score(9, 25));
    assert_eq!(8317, find_highest_score(10, 1618));
    assert_eq!(146373, find_highest_score(13, 7999));
    assert_eq!(2764, find_highest_score(17, 1104));
    assert_eq!(54718, find_highest_score(21, 6111));
    assert_eq!(37305, find_highest_score(30, 5807));
}
