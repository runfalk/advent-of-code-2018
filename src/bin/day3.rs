use aoc::{buf_reader_from_arg, parse_lines};
use lazy_static::lazy_static;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl FromStr for Rectangle {
    type Err = ();

    fn from_str(rect: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref rect_re: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        let caps = rect_re.captures(rect).ok_or(())?;

        let id = caps.get(1).ok_or(())?.as_str().parse::<usize>().or(Err(()))?;
        let x = caps.get(2).ok_or(())?.as_str().parse::<usize>().or(Err(()))?;
        let y = caps.get(3).ok_or(())?.as_str().parse::<usize>().or(Err(()))?;
        let width = caps.get(4).ok_or(())?.as_str().parse::<usize>().or(Err(()))?;
        let height = caps.get(5).ok_or(())?.as_str().parse::<usize>().or(Err(()))?;

        Ok(Rectangle {id, x, y, width, height})
    }
}

struct Fabric {
    tiles: [u8; 1_000_000],
}

impl Fabric {
    const WIDTH: usize = 1_000;

    fn new() -> Self {
        Self { tiles: [0; 1_000_000] }
    }

    fn alloc_patch(&mut self, rect: &Rectangle) -> u8 {
        let mut num_overlaps = 0;
        for x in rect.x..rect.width + rect.x {
            for y in rect.y..rect.height + rect.y {
                let tile = self.tiles.get_mut(x + y * Self::WIDTH).unwrap();

                if *tile > num_overlaps {
                    num_overlaps = *tile;
                }

                *tile += 1;
            }
        }
        num_overlaps
    }

    fn num_overlaps(&self) -> usize {
        self.tiles.iter().filter(|t| **t > 1).count()
    }
}

fn part_ab(rects: &Vec<Rectangle>) -> (usize, usize) {
    let mut fabric = Fabric::new();
    for rect in rects.iter() {
        fabric.alloc_patch(rect);
    }

    let num_overlaps = fabric.num_overlaps();

    for rect in rects.iter() {
        if fabric.alloc_patch(rect) == 1 {
            return (num_overlaps, rect.id);
        }
    }

    panic!("Found no non-overlapping rectangles");
}


fn main() {
    let rects = parse_lines::<Rectangle>(buf_reader_from_arg().unwrap()).collect::<Vec<Rectangle>>();
    let (a, b) = part_ab(&rects);
    println!("Answer A: {}", a);
    println!("Answer B: {}", b);
}

#[test]
fn test_parse_rectangle() {
    assert_eq!("#1 @ 1,3: 4x4".parse::<Rectangle>(), Ok(Rectangle { id: 1, x: 1, y: 3, width: 4, height: 4}));
    assert_eq!("#2 @ 3,1: 4x4".parse::<Rectangle>(), Ok(Rectangle { id: 2, x: 3, y: 1, width: 4, height: 4}));
    assert_eq!("#3 @ 5,5: 2x2".parse::<Rectangle>(), Ok(Rectangle { id: 3, x: 5, y: 5, width: 2, height: 2}));
}

#[test]
fn test_part_ab() {
    let data = vec![
        "#1 @ 1,3: 4x4".parse::<Rectangle>().unwrap(),
        "#2 @ 3,1: 4x4".parse::<Rectangle>().unwrap(),
        "#3 @ 5,5: 2x2".parse::<Rectangle>().unwrap(),
    ];
    assert_eq!(part_ab(&data), (4, 3));
}
