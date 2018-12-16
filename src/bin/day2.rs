use aoc::{buf_reader_from_arg, parse_lines};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ChecksumPart {
    has_doubles: bool,
    has_triples: bool,
}

impl FromStr for ChecksumPart {
    type Err = ();

    fn from_str(box_id: &str) -> Result<Self, Self::Err> {
        let mut m = HashMap::new();
        let mut num_doubles = 0;
        let mut num_triples = 0;

        for c in box_id.chars() {
            let count = m.entry(c).or_insert(0);
            *count += 1;

            if *count == 4 {
                num_triples -= 1;
            } else if *count == 3 {
                num_triples += 1;
                num_doubles -= 1;
            } else if *count == 2 {
                num_doubles += 1;
            }
        }

        Ok(Self {
            has_doubles: num_doubles > 0,
            has_triples: num_triples > 0,
        })
    }
}

fn part_a(parts: impl Iterator<Item = ChecksumPart>) -> usize {
    let mut doubles = 0;
    let mut triples = 0;

    for p in parts {
        if p.has_doubles {
            doubles += 1;
        }
        if p.has_triples {
            triples += 1;
        }
    }
    doubles * triples
}

fn part_b(box_ids: impl Iterator<Item = String>) -> String {
    let box_ids = box_ids.collect::<Vec<String>>();

    for (i, current) in box_ids.iter().enumerate() {
        for next in box_ids[i + 1..].iter() {
            let mut common_letters = String::with_capacity(current.len());
            for (current_char, next_char) in current.chars().zip(next.chars()) {
                if current_char == next_char {
                    common_letters.push(current_char);
                }
            }
            if current.len() == common_letters.len() + 1 {
                return common_letters;
            }
        }
    }
    panic!("No similar boxes found");
}

fn main() {
    println!(
        "Answer A: {}",
        part_a(parse_lines::<ChecksumPart>(buf_reader_from_arg().unwrap()))
    );
    println!(
        "Answer B: {}",
        part_b(parse_lines::<String>(buf_reader_from_arg().unwrap()))
    );
}

#[test]
fn test_checksum_part() {
    assert_eq!(
        "abcdef".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: false,
            has_triples: false
        }
    );
    assert_eq!(
        "bababc".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: true,
            has_triples: true
        }
    );
    assert_eq!(
        "abbcde".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: true,
            has_triples: false
        }
    );
    assert_eq!(
        "abcccd".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: false,
            has_triples: true
        }
    );
    assert_eq!(
        "aabcdd".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: true,
            has_triples: false
        }
    );
    assert_eq!(
        "abcdee".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: true,
            has_triples: false
        }
    );
    assert_eq!(
        "ababab".parse::<ChecksumPart>().unwrap(),
        ChecksumPart {
            has_doubles: false,
            has_triples: true
        }
    );
}

#[test]
fn test_part_b() {
    let box_ids = vec![
        "abcde".to_owned(),
        "fghij".to_owned(),
        "klmno".to_owned(),
        "pqrst".to_owned(),
        "fguij".to_owned(),
        "axcye".to_owned(),
        "wvxyz".to_owned(),
    ];
    assert_eq!(part_b(box_ids.into_iter()), "fgij");
}
