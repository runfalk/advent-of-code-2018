use aoc::{buf_reader_from_arg, parse_lines};
use chrono::{NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::str::FromStr;

struct SleepPattern {
    by_min: [usize; 60],
    sleep_start: Option<usize>,
    num_mins: usize,
}

impl SleepPattern {
    fn new() -> Self {
        Self {
            by_min: [0; 60],
            sleep_start: None,
            num_mins: 0,
        }
    }
    fn go_to_sleep(&mut self, min: usize) {
        if self.sleep_start.is_some() {
            panic!("Guard is already asleep");
        }
        self.sleep_start = Some(min);
    }

    fn wake_up(&mut self, min: usize) {
        if self.sleep_start.is_none() {
            panic!("Guard is not asleep");
        }
        for min in self.sleep_start.unwrap()..min {
            self.by_min[min] += 1;
            self.num_mins += 1;
        }
        self.sleep_start = None;
    }

    fn most_asleep(&self) -> Option<(usize, usize)> {
        if self.num_mins == 0 {
            return None;
        }

        self.by_min
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|(_, count)| *count)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GuardEventType {
    Begin(usize),
    Asleep,
    Awake,
}

#[derive(Debug, Eq, PartialEq)]
struct GuardEvent {
    timestamp: NaiveDateTime,
    event_type: GuardEventType,
}

impl FromStr for GuardEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref guard_re: Regex = Regex::new(
                r"^\[([^\]]+)\] (?:Guard #(\d+) begins shift|(falls asleep)|(wakes up))$"
            )
            .unwrap();
        }

        let captures = guard_re.captures(s).ok_or(())?;
        let ts = NaiveDateTime::parse_from_str(captures.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M")
            .unwrap();

        if let Some(string_id) = captures.get(2) {
            Ok(Self {
                timestamp: ts,
                event_type: GuardEventType::Begin(string_id.as_str().parse::<usize>().or(Err(()))?),
            })
        } else if captures.get(3) != None {
            Ok(Self {
                timestamp: ts,
                event_type: GuardEventType::Asleep,
            })
        } else {
            Ok(Self {
                timestamp: ts,
                event_type: GuardEventType::Awake,
            })
        }
    }
}

fn part_a(guard_patterns: &HashMap<usize, SleepPattern>) -> usize {
    let (id, _, min) = guard_patterns
        .iter()
        .map(|(k, v)| {
            let most_asleep = v.most_asleep().unwrap_or((0, 0));
            (k, v.num_mins, most_asleep.0)
        })
        .max_by_key(|(_, num_mins, _)| *num_mins)
        .unwrap();

    id * min
}

fn part_b(guard_patterns: &HashMap<usize, SleepPattern>) -> usize {
    let (id, _, min) = guard_patterns
        .iter()
        .map(|(k, v)| {
            let most_asleep = v.most_asleep().unwrap_or((0, 0));
            (k, most_asleep.1, most_asleep.0)
        })
        .max_by_key(|(_, most_sleep, _)| *most_sleep)
        .unwrap();

    id * min
}

fn main() {
    let mut guard_events =
        parse_lines::<String>(buf_reader_from_arg().unwrap()).collect::<Vec<String>>();
    guard_events.sort();

    let current_id: Option<NonZeroUsize> = None;
    let it = guard_events
        .into_iter()
        .map(|e| e.parse::<GuardEvent>().unwrap())
        .scan(current_id, |current_id, event| {
            if let GuardEventType::Begin(id) = event.event_type {
                *current_id = NonZeroUsize::new(id);
            }
            Some((*current_id, event))
        });

    let mut guard_patterns = HashMap::new();
    for (id, event) in it {
        let entry = guard_patterns
            .entry(id.unwrap().get())
            .or_insert(SleepPattern::new());
        match event.event_type {
            GuardEventType::Begin(_) => continue,
            GuardEventType::Asleep => entry.go_to_sleep(event.timestamp.minute() as usize),
            GuardEventType::Awake => entry.wake_up(event.timestamp.minute() as usize),
        }
    }

    println!("Answer A: {}", part_a(&guard_patterns));
    println!("Answer B: {}", part_b(&guard_patterns));
}

#[test]
fn test_parse_guard_event() {
    assert_eq!(
        "[1518-11-01 00:00] Guard #10 begins shift".parse::<GuardEvent>(),
        Ok(GuardEvent {
            timestamp: "1518-11-01 00:00".into(),
            event_type: GuardEventType::Begin(10)
        }),
    );
    assert_eq!(
        "[1518-11-01 00:05] falls asleep".parse::<GuardEvent>(),
        Ok(GuardEvent {
            timestamp: "1518-11-01 00:05".into(),
            event_type: GuardEventType::Asleep
        }),
    );
    assert_eq!(
        "[1518-11-01 00:25] wakes up".parse::<GuardEvent>(),
        Ok(GuardEvent {
            timestamp: "1518-11-01 00:25".into(),
            event_type: GuardEventType::Awake
        }),
    );
}
