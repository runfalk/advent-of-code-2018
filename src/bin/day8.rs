use aoc::{buf_reader_from_arg, parse_lines};
use std::collections::BTreeMap;

fn part_a(mut parts: impl Iterator<Item = u8>) -> usize {
    fn rec(parts: &mut impl Iterator<Item = u8>) -> usize {
        let num_children = parts.next().unwrap();
        let num_metadata = parts.next().unwrap();

        let mut metadata_sum = 0;
        for _ in 0..num_children {
            metadata_sum += rec(parts);
        }

        for _ in 0..num_metadata {
            metadata_sum += parts.next().unwrap() as usize;
        }
        metadata_sum
    }
    rec(&mut parts)
}

fn part_b(mut parts: impl Iterator<Item = u8>) -> usize {
    fn rec(parts: &mut impl Iterator<Item = u8>) -> usize {
        let num_children = parts.next().unwrap();
        let num_metadata = parts.next().unwrap();

        let mut child_values = BTreeMap::new();
        for i in 0..num_children {
            // Child node indexing starts at one
            child_values.insert(i + 1, rec(parts));
        }

        let mut value = 0;
        if num_children > 0 {
            for _ in 0..num_metadata {
                let i = parts.next().unwrap();
                if let Some(v) = child_values.get(&i) {
                    value += v;
                }
            }
        } else {
            for _ in 0..num_metadata {
                value += parts.next().unwrap() as usize;
            }
        }
        value
    }
    rec(&mut parts)
}

fn main() {
    let line = parse_lines::<String>(buf_reader_from_arg().unwrap()).next().unwrap();
    let license = line
        .split(" ")
        .map(|x| x.parse::<u8>().unwrap());

    println!("Answer A: {}", part_a(license.clone()));
    println!("Answer B: {}", part_b(license));
}

#[test]
fn test_a() {
    let license = vec![2u8, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(part_a(license.into_iter()), 138);
}


#[test]
fn test_b() {
    let license = vec![2u8, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(part_b(license.into_iter()), 66);
}
