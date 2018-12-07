use aoc::{buf_reader_from_arg, parse_lines};
use std::collections::HashSet;

fn part_a(changes: impl Iterator<Item = i32>) -> i32 {
    changes.sum()
}

fn part_b(changes: impl Iterator<Item = i32>) -> i32 {
    let mut current: i32 = 0;
    let mut memory = HashSet::new();
    memory.insert(current);

    for change in changes.collect::<Vec<i32>>().iter().cycle() {
        current += change;
        if !memory.insert(current) {
            return current;
        }
    }
    unreachable!();
}

fn main() {
    println!("Answer A: {}", part_a(parse_lines::<i32>(buf_reader_from_arg().unwrap())));
    println!("Answer B: {}", part_b(parse_lines::<i32>(buf_reader_from_arg().unwrap())));
}

#[test]
fn test_a() {
    assert_eq!(part_a(vec![1i32, -2, 3, 1].into_iter()), 3);
}

#[test]
fn test_b() {
    assert_eq!(part_b(vec![1i32, -2, 3, 1].into_iter()), 2);
}
