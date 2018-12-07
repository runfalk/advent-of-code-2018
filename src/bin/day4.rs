use aoc::{buf_reader_from_arg, parse_lines};
use lazy_static::lazy_static;
use std::str::FromStr;
use std::num::NonZeroUsize;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
enum GuardEventType {
    Begin(usize),
    Asleep,
    Awake,
}

#[derive(Debug, Eq, PartialEq)]
struct GuardEvent {
    timestamp: String,
    event_type: GuardEventType,
}

impl FromStr for GuardEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref guard_re: Regex = Regex::new(r"^\[([^\]]+)\] (?:Guard #(\d+) begins shift|(falls asleep)|(wakes up))$").unwrap();
        }

        let captures = guard_re.captures(s).ok_or(())?;
        let ts = captures.get(1).unwrap().as_str().to_owned();

        if let Some(string_id) = captures.get(2) {
            Ok(Self {timestamp: ts, event_type: GuardEventType::Begin(string_id.as_str().parse::<usize>().or(Err(()))?)})
        } else if captures.get(3) != None {
            Ok(Self {timestamp: ts, event_type: GuardEventType::Asleep})
        } else {
            Ok(Self {timestamp: ts, event_type: GuardEventType::Awake})
        }
    }
}

fn part_a(guard_events: impl Iterator<Item = GuardEvent>) -> usize {
    // let current_id: Option<NonZeroUsize> = None;
    // let it = guard_events.scan(current_id, |current_id, event| {
    //     if let GuardEventType::Begin(id) = event {
    //         *current_id = id;
    //     }
    //     (*current_id, event)
    // });

    // let mut guard_patterns = HashMap::new();
    // for (id, event) in it {
    // }
    0
}

fn main() {
    let mut guard_events = parse_lines::<String>(buf_reader_from_arg().unwrap()).collect::<Vec<String>>();
    guard_events.sort();

    println!("Answer A: {}", part_a(guard_events.into_iter().map(|e| e.parse::<GuardEvent>().unwrap())));
}

#[test]
fn test_parse_guard_event() {
    assert_eq!(
        "[1518-11-01 00:00] Guard #10 begins shift".parse::<GuardEvent>(),
        Ok(GuardEvent { timestamp: "1518-11-01 00:00".into(), event_type: GuardEventType::Begin(10)}),
    );
    assert_eq!(
        "[1518-11-01 00:05] falls asleep".parse::<GuardEvent>(),
        Ok(GuardEvent { timestamp: "1518-11-01 00:05".into(), event_type: GuardEventType::Asleep}),
    );
    assert_eq!(
        "[1518-11-01 00:25] wakes up".parse::<GuardEvent>(),
        Ok(GuardEvent { timestamp: "1518-11-01 00:25".into(), event_type: GuardEventType::Awake}),
    );
}
