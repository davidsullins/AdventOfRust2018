// day3.rs
// overlapping rectangles

use bit_vec::BitVec;
use lazy_static::*;
use regex::Regex;
use std::io::BufRead;

fn main() {
    let claims: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_line(&l.expect("Failed to read line")))
        .collect();
    println!("part 1: {}", count_claimed_by_at_least_two(&claims));
    println!("part 2: {}", find_non_overlapping_id(&claims));
}

// part 1
fn count_claimed_by_at_least_two(claims: &[Rect]) -> usize {
    let mut cloth = Cloth::new();
    for rect in claims {
        for x in rect.x1..rect.x2 {
            for y in rect.y1..rect.y2 {
                cloth.claim_point(x, y);
            }
        }
    }
    cloth.count_claimed_by_at_least_two()
}

// Problem description claims that overall square is at _least_ 1000 inches per side,
// but it appears to be less than that. Perhaps they meant at _most_ 1000 inches?
struct Cloth([u8; 1_000_000]);

impl Cloth {
    fn new() -> Cloth {
        Cloth([0; 1_000_000])
    }

    fn claim_point(&mut self, x: usize, y: usize) {
        let pos = y * 1000 + x;
        self.0[pos] = std::cmp::min(self.0[pos] + 1, 2);
    }

    fn count_claimed_by_at_least_two(&self) -> usize {
        self.0.iter().filter(|&&x| x > 1).count()
    }
}

// x/y position from top left corner
// not that it matters, the math would work out the same in any other way too
#[derive(PartialEq, Debug)]
struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn parse_line(line: &str) -> Rect {
    lazy_static! {
        static ref RE_CLAIM: Regex = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    if let Some(caps) = RE_CLAIM.captures(line) {
        let x1: usize = caps[1].parse().unwrap();
        let y1: usize = caps[2].parse().unwrap();
        let width: usize = caps[3].parse().unwrap();
        let height: usize = caps[4].parse().unwrap();
        let x2: usize = x1 + width;
        let y2: usize = y1 + height;
        Rect { x1, y1, x2, y2 }
    } else {
        panic!("Invalid line!")
    }
}

#[test]
fn test_parse_line() {
    let claim = Rect {
        x1: 3,
        y1: 2,
        x2: 3 + 5,
        y2: 2 + 4,
    };
    assert_eq!(parse_line("#123 @ 3,2: 5x4"), claim);
}

#[test]
fn test_count_claimed_by_at_least_two() {
    let input = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
    let claims: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
    assert_eq!(4, count_claimed_by_at_least_two(&claims));
}

// part 2
fn is_overlap(r1: &Rect, r2: &Rect) -> bool {
    // overlap in one axis if max of mins < min of maxes
    // axis-aligned rectangles overlap iff all axes overlap
    r1.x1.max(r2.x1) < r1.x2.min(r2.x2) && r1.y1.max(r2.y1) < r1.y2.min(r2.y2)
}

fn find_non_overlapping_id(rects: &[Rect]) -> usize {
    let mut overlaps = BitVec::from_elem(rects.len(), false);
    for (r1_idx, r1) in rects.iter().enumerate() {
        // avoid redundant tests and avoid comparison with self
        for (j, r2) in rects[r1_idx + 1..].iter().enumerate() {
            let r2_idx = r1_idx + j + 1;
            if is_overlap(r1, r2) {
                overlaps.set(r1_idx, true);
                overlaps.set(r2_idx, true);
            }
        }
    }

    for (i, has_overlap) in overlaps.iter().enumerate() {
        if !has_overlap {
            // puzzle starts counting claims at 1, so offset by 1
            return i + 1;
        }
    }

    0
}

#[test]
fn test_is_overlap() {
    let input = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
    let claims: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
    assert!(is_overlap(&claims[0], &claims[1]));
    assert!(!is_overlap(&claims[0], &claims[2]));
    assert!(!is_overlap(&claims[1], &claims[2]));
}

#[test]
fn test_find_non_overlapping_id() {
    let input = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
    let claims: Vec<_> = input.iter().map(|s| parse_line(s)).collect();
    assert_eq!(find_non_overlapping_id(&claims), 3);
}
