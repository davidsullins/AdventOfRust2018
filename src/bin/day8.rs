// day8.rs
// parse license file

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let data = parse_input(&input);
    println!("part 1: {}", sum_metadata(&data));
    println!("part 2: {}", compute_root_node_value(&data));
}

// part 1
fn parse_input(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .map(|num_str| num_str.parse::<usize>().unwrap())
        .collect()
}

// Sum the metadata values
fn sum_metadata(data: &[usize]) -> usize {
    sum_metadata_r(&mut data.iter())
}

fn sum_metadata_r(data: &mut std::slice::Iter<usize>) -> usize {
    let mut sum = 0;
    // This function could panic anywhere if the input data is bad
    let &child_node_count = data.next().unwrap();
    let &metadata_count = data.next().unwrap();
    for _ in 0..child_node_count {
        sum += sum_metadata_r(data);
    }
    sum + data.take(metadata_count).sum::<usize>()
}

#[test]
fn test_sum_metadata() {
    let v = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(138, sum_metadata(&v));
}

#[test]
fn test_parse_input() {
    assert_eq!(
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
        parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
    );
}

// part 2
// Sum the metadata values
fn compute_root_node_value(data: &[usize]) -> usize {
    compute_node_value(&mut data.iter())
}

fn compute_node_value(data: &mut std::slice::Iter<usize>) -> usize {
    // This function could panic anywhere if the input data is bad
    let &child_node_count = data.next().unwrap();
    let &metadata_count = data.next().unwrap();
    let mut child_node_values = vec![];
    for _ in 0..child_node_count {
        child_node_values.push(compute_node_value(data));
    }
    if child_node_values.is_empty() {
        data.take(metadata_count).sum()
    } else {
        data.take(metadata_count)
            .map(|&child_idx| child_node_values.get(child_idx - 1).unwrap_or(&0))
            .sum()
    }
}

#[test]
fn test_compute_root_node_value() {
    let v = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(66, compute_root_node_value(&v));
}
