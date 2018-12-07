use aoc::{buf_reader_from_arg, parse_lines};

fn can_react(a: char, b: char) -> bool {
    match (a.is_uppercase(), b.is_uppercase()) {
        (true, false) | (false, true) => a.eq_ignore_ascii_case(&b),
        (true, true) | (false, false) => false,
    }
}

fn react(units: impl Iterator<Item = char>) -> String {
    let mut out = String::new();
    for curr in units {
        if let Some(prev) = out.chars().last() {
            if can_react(curr, prev) {
                out.pop();
                continue;
            }
        }
        out.push(curr);
    }
    out
}

fn part_a(units: impl Iterator<Item = char>) -> usize {
    react(units).len()
}

fn part_b(polymer: String) -> usize {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|excl| {
        react(polymer.chars().filter(|u| !u.eq_ignore_ascii_case(&excl))).len()
    }).min().unwrap()
}

fn main() {
    let polymer = parse_lines::<String>(buf_reader_from_arg().unwrap()).next().unwrap();
    println!("Answer A: {}", part_a(polymer.chars()));
    println!("Answer B: {}", part_b(polymer));
}

#[test]
fn test_a() {
    assert_eq!(part_a("aA".chars()), 0);
    assert_eq!(part_a("abBA".chars()), 0);
    assert_eq!(part_a("abAB".chars()), 4);
    assert_eq!(part_a("aabAAB".chars()), 6);
    assert_eq!(part_a("dabAcCaCBAcCcaDA".chars()), 10);
}

#[test]
fn test_b() {
    assert_eq!(part_b("dabAcCaCBAcCcaDA".into()), 4);
}
