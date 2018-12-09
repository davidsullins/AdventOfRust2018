// day2.rs
// find nearly-matching box IDs

use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect();
    let box_ids: Vec<&str> = input.iter().map(|s| s as &str).collect();
    println!("part 1: {}", calc_checksum(&box_ids));
    println!("part 2: {}", find_common_letters(&box_ids).unwrap());
}

// part 1
fn count_exactly_n(box_ids: &[&str], n: usize) -> usize {
    let mut count = 0;
    for box_id in box_ids.iter() {
        let mut letter_count = HashMap::new();
        for c in box_id.chars() {
            let count = letter_count.get(&c).unwrap_or(&0);
            letter_count.insert(c, count + 1);
        }
        if letter_count.values().any(|&x| x == n) {
            count += 1;
        }
    }
    count
}

fn calc_checksum(box_ids: &[&str]) -> usize {
    count_exactly_n(box_ids, 2) * count_exactly_n(box_ids, 3)
}

#[test]
fn test_count_exactly_two() {
    let v = [
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    assert_eq!(count_exactly_n(&v, 2), 4);
}

#[test]
fn test_count_exactly_three() {
    let v = [
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    assert_eq!(count_exactly_n(&v, 3), 3);
}

#[test]
fn test_calc_checksum() {
    let v = [
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    assert_eq!(calc_checksum(&v), 12);
}

// part 2
// I brute forced it. It runs so quickly it didn't seem worth it to make it efficient.
// differ_by_one_char could use an early out (unless the compiler is really smart)
// There's probably a great algorithm that doesn't require O(N^2) comparing every
// string with every string but I didn't look for one.
fn find_common_letters(box_ids: &[&str]) -> Option<String> {
    if let Some((box1, box2)) = find_fabric_boxes(box_ids) {
        return Some(
            box1.chars()
                .zip(box2.chars())
                .filter_map(|(x, y)| if x == y { Some(x) } else { None })
                .collect(),
        );
    }

    None
}

#[test]
fn test_find_common_letters() {
    let v = [
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    assert_eq!(find_common_letters(&v).unwrap(), "fgij");
}

fn find_fabric_boxes<'a>(box_ids: &[&'a str]) -> Option<(&'a str, &'a str)> {
    for s1 in box_ids {
        for s2 in box_ids {
            if differ_by_one_char(s1, s2) {
                return Some((s1, s2));
            }
        }
    }
    None
}

fn differ_by_one_char(s1: &str, s2: &str) -> bool {
    s1.chars().zip(s2.chars()).filter(|(x, y)| x != y).count() == 1
}

#[test]
fn test_find_box_ids() {
    let v = [
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    assert_eq!(find_fabric_boxes(&v).unwrap(), ("fghij", "fguij"));
}

#[test]
fn test_differ_by_one_char() {
    assert!(differ_by_one_char("fghij", "fguij"));
    assert!(!differ_by_one_char("abcde", "axcye"));
}
