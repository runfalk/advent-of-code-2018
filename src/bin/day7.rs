use aoc::{buf_reader_from_arg, parse_lines};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

fn parse_dep(s: String) -> (char, char) {
    lazy_static! {
        static ref guard_re: Regex =
            Regex::new(r"^Step (\w) must be finished before step (\w)").unwrap();
    }
    let captures = guard_re.captures(&s).unwrap();
    (
        captures[1].parse::<char>().unwrap(),
        captures[2].parse::<char>().unwrap(),
    )
}

fn part_a(deps: impl Iterator<Item = (char, char)>) -> String {
    let mut available = BTreeSet::new();
    let mut dep_to_steps = BTreeMap::new();
    let mut step_to_deps = BTreeMap::new();

    for (dep, step) in deps {
        dep_to_steps
            .entry(dep)
            .or_insert(BTreeSet::new())
            .insert(step);
        step_to_deps
            .entry(step)
            .or_insert(BTreeSet::new())
            .insert(dep);

        // Mark all dependencies as candidates for being available
        available.insert(dep);
    }

    // Only dependencies which have no dependencies themselves are available
    // at the start
    for step in step_to_deps.keys() {
        available.remove(step);
    }

    let mut done = BTreeSet::new();
    let mut out = String::with_capacity(step_to_deps.len());
    while !available.is_empty() {
        // Take the lowest available letter and put it in the output
        let current = available.iter().next().unwrap().clone();
        available.remove(&current);
        out.push(current);
        done.insert(current);

        for step in dep_to_steps
            .entry(current)
            .or_insert(BTreeSet::new())
            .iter()
        {
            if step_to_deps[step].is_subset(&done) {
                available.insert(step.clone());
            }
        }
    }

    out
}

fn main() {
    let deps = parse_lines::<String>(buf_reader_from_arg().unwrap()).map(parse_dep);
    println!("Answer A: {}", part_a(deps));
}

#[test]
fn test_parse_dep() {
    assert_eq!(
        parse_dep("Step C must be finished before step A can begin.".into()),
        ('C', 'A')
    );
}

#[test]
fn test_part_a() {
    let data = vec![
        ('C', 'A'),
        ('C', 'F'),
        ('A', 'B'),
        ('A', 'D'),
        ('B', 'E'),
        ('D', 'E'),
        ('F', 'E'),
    ];
    assert_eq!(part_a(data.into_iter()), "CABDFE");
}
