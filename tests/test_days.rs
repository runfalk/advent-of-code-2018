use std::ffi::OsStr;
use std::process::Command;
use std::str;

fn run_day<I, S>(day: usize, args: I) -> Result<(String, Option<String>), ()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("cargo")
        .args(&["run", "-q", "--bin", &format!("day{}", day)])
        .args(args)
        .output()
        .or(Err(()))?;
    let stdout = str::from_utf8(&output.stdout).or(Err(()))?;

    let mut a = None;
    let mut b = None;
    for line in stdout.lines() {
        match line.chars().take(8).last().unwrap() {
            'A' => a = Some(line.chars().skip(10).collect()),
            'B' => b = Some(line.chars().skip(10).collect()),
            c @ _ => panic!("Unknown answer letter: {:?}", c),
        }
    }

    Ok((a.unwrap(), b))
}

fn a(a: &str) -> (String, Option<String>) {
    (a.to_owned(), None)
}

fn ab(a: &str, b: &str) -> (String, Option<String>) {
    (a.to_owned(), Some(b.to_owned()))
}

#[test]
fn test_day1() {
    assert_eq!(run_day(1, &["data/day1.txt"]).unwrap(), ab("585", "83173"));
}

#[test]
fn test_day2() {
    assert_eq!(
        run_day(2, &["data/day2.txt"]).unwrap(),
        ab("4712", "lufjygedpvfbhftxiwnaorzmq")
    );
}

#[test]
fn test_day3() {
    assert_eq!(run_day(3, &["data/day3.txt"]).unwrap(), ab("105231", "164"));
}

#[test]
fn test_day4() {
    assert_eq!(
        run_day(4, &["data/day4.txt"]).unwrap(),
        ab("19025", "23776")
    );
}

#[test]
fn test_day5() {
    assert_eq!(run_day(5, &["data/day5.txt"]).unwrap(), ab("10888", "6952"));
}

#[test]
fn test_day7() {
    assert_eq!(
        run_day(7, &["data/day7.txt"]).unwrap(),
        a("JKNSTHCBGRVDXWAYFOQLMPZIUE")
    );
}

#[test]
fn test_day8() {
    assert_eq!(
        run_day(8, &["data/day8.txt"]).unwrap(),
        ab("37905", "33891")
    );
}

#[test]
fn test_day9() {
    assert_eq!(
        run_day(9, &["411", "71170"]).unwrap(),
        ab("425688", "3526561003")
    );
}

#[test]
fn test_day11() {
    assert_eq!(run_day(11, &["5535"]).unwrap(), a("19,41"));
}

#[test]
fn test_day14() {
    assert_eq!(
        run_day(14, &["864801"]).unwrap(),
        ab("1611732174", "20279772")
    );
}

#[test]
fn test_day16() {
    assert_eq!(run_day(16, &["data/day16.txt"]).unwrap(), ab("607", "577"));
}
