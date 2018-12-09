// day1.rs
// sum frequencies

use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    println!("part 1: {}", calc_freq(&input));
    println!("part 2: {}", find_first_repeated_freq(&input));
}

// part 1
fn calc_freq(s: &str) -> i32 {
    s.split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .sum()
}

#[test]
fn test_calc_freq() {
    assert_eq!(calc_freq("+1\n-2\n+3\n+1"), 3);
    assert_eq!(calc_freq("+1\n+1\n+1"), 3);
    assert_eq!(calc_freq("+1\n+1\n-2"), 0);
    assert_eq!(calc_freq("-1\n-2\n-3"), -6);
}

// part 2
fn parse_changes(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

#[test]
fn test_parse_changes() {
    assert_eq!(parse_changes("+1 -2 +3 +1"), vec![1, -2, 3, 1]);
    assert_eq!(parse_changes("+1 +1 +1"), vec![1, 1, 1]);
    assert_eq!(parse_changes("+1 +1 -2"), vec![1, 1, -2]);
    assert_eq!(parse_changes("-1 -2 -3"), vec![-1, -2, -3]);
}

fn find_first_repeated_freq(s: &str) -> i32 {
    let mut total = 0;
    let mut reached_freqs = HashSet::new();
    reached_freqs.insert(total);

    for x in parse_changes(&s).iter().cycle() {
        total += *x;
        if reached_freqs.contains(&total) {
            break;
        }
        reached_freqs.insert(total);
    }

    total
}

#[test]
fn test_find_first_repeated_freq() {
    assert_eq!(find_first_repeated_freq("+1 -2 +3 +1"), 2);
    assert_eq!(find_first_repeated_freq("+1 -1"), 0);
    assert_eq!(find_first_repeated_freq("+3 +3 +4 -2 -4"), 10);
    assert_eq!(find_first_repeated_freq("-6 +3 +8 +5 -6"), 5);
    assert_eq!(find_first_repeated_freq("+7 +7 -2 -7 -4"), 14);
}
