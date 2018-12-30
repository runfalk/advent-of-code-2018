use aoc::{buf_reader_from_arg, parse_lines};
use std::fmt;
use std::collections::VecDeque;
use std::str::FromStr;

struct PotArea(u8);

impl PotArea {
    fn new(l2: bool, l: bool, c: bool, r: bool, r2: bool) -> Self {
        let mut index = 0u8;
        if l2 {
            index |= 1 << 4;
        }
        if l {
            index |= 1 << 3;
        }
        if c {
            index |= 1 << 2;
        }
        if r {
            index |= 1 << 1;
        }
        if r2 {
            index |= 1;
        }
        PotArea(index)
    }

    fn advance(&self, r2: bool) -> Self {
        PotArea((self.0 << 1) & 0b11111 | if r2 { 1 } else { 0 })
    }
}

impl Default for PotArea {
    fn default() -> Self {
        PotArea(0)
    }
}

impl fmt::Debug for PotArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr = String::with_capacity(5);
        for i in 0..5 {
            repr.push(if (self.0 >> (4 - i)) & 1 == 1 { '#' } else { '.' });
        }
        write!(f, "PotArea<{}>", repr)
    }
}

impl FromStr for PotArea {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pots: Vec<_> = s[0..5].chars().map(|x| x == '#').collect();
        Ok(PotArea::new(pots[0], pots[1], pots[2], pots[3], pots[4]))
    }
}

#[derive(Debug)]
struct PotMap {
    map: [bool; 32],
}

impl PotMap {
    fn from_iter(pot_areas: impl Iterator<Item = PotArea>) -> Self {
        let mut map = [false; 32];
        for conf in pot_areas {
            assert!(
                conf.0 > 0,
                "A completely empty area must not result in a pot",
            );
            map[conf.0 as usize] = true;
        }
        PotMap { map }
    }

    fn can_grow(&self, pot_area: &PotArea) -> bool {
        self.map[pot_area.0 as usize]
    }
}

#[derive(Clone)]
struct Pots {
    offset: isize,
    pots: VecDeque<bool>,
}

impl Pots {
    fn from_iter(offset: isize, it: impl Iterator<Item = bool>) -> Self {
        let pots = it.collect();
        Pots { offset, pots }.normalize()
    }

    fn normalize(mut self) -> Self {
        while self.pots.len() > 0 && self.pots.front() != Some(&true) {
            self.pots.pop_front();
            self.offset += 1;
        }

        while self.pots.len() > 0 && self.pots.back() != Some(&true) {
            self.pots.pop_back();
        }

        self
    }

    fn has_pot(&self, index: isize) -> bool {
        if index < self.offset {
            return false;
        }
        *self.pots.get((index - self.offset) as usize).unwrap_or(&false)
    }

    fn sum(&self) -> isize {
        self.pots.iter().enumerate().filter_map(|(i, has_pot)| {
            if *has_pot {
                Some(i as isize + self.offset)
            } else {
                None
            }
        }).sum()
    }
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr: String = self.pots.iter().map(|&has_pot| if has_pot {
            '#'
        } else {
            '.'
        }).collect();
        write!(f, "Pots<{}, {}>", self.offset, repr)
    }
}

fn simulate(gens: usize, pots: Pots, pot_map: &PotMap) -> Pots {
    let mut history: Vec<Pots> = Vec::new();
    history.push(pots);

    for _ in 0..gens {
        let prev = &history[history.len() - 1];
        let indices = prev.offset - 2..prev.offset + prev.pots.len() as isize + 2;
        let curr = Pots::from_iter(indices.start, indices.scan(PotArea::default(), |pot_area, i| {
            *pot_area = pot_area.advance(prev.has_pot(i + 2));
            Some(pot_map.can_grow(pot_area))
        }));

        // See if we have observed this particular plant configuration before.
        // This is only sensitive to the pattern, not the offset.
        for (prev_gen, prev) in history.iter().enumerate().rev() {
            if prev.pots != curr.pots {
                continue;
            }

            let curr_gen = history.len();

            // How much the offset increases per cycle
            let offset_inc_per_cycle = curr.offset - prev.offset;

            // TODO: Cycle lengths other than 1 are not handled properly
            let cycle_length = curr_gen - prev_gen;
            assert!(cycle_length == 1, "Cycle length must be no longer than 1");
            let num_cycles = (gens - prev_gen) / cycle_length;

            let interpolated_pots = Pots {
                offset: prev.offset + (num_cycles as isize * offset_inc_per_cycle),
                pots: history[prev_gen].pots.clone(),
            };
            return interpolated_pots
        }

        history.push(curr);
    }

    history.into_iter().last().unwrap()
}

fn part_a(pots: Pots, pot_map: &PotMap) -> isize {
    simulate(20, pots, pot_map).sum()
}

fn part_b(pots: Pots, pot_map: &PotMap) -> isize {
    simulate(50_000_000_000, pots, pot_map).sum()
}

fn main() {
    let mut data = parse_lines::<String>(buf_reader_from_arg().unwrap());

    let initial_state_str = data.next().unwrap();
    data.next();
    let pots = Pots::from_iter(0, initial_state_str[15..].chars().map(|x| x == '#'));

    let pot_map = PotMap::from_iter(data
        .filter(|x| x.chars().last() == Some('#'))
        .map(|x| x.parse::<PotArea>().unwrap())
    );

    println!("Answer A: {}", part_a(pots.clone(), &pot_map));
    println!("Answer B: {}", part_b(pots, &pot_map));
}

#[test]
fn test_part_a() {
    let state = "#..#.#..##......###...###";
    let pots = Pots::from_iter(0, state.chars().map(|x| x == '#'));

    let patterns = vec![
        "...##",
        "..#..",
        ".#...",
        ".#.#.",
        ".#.##",
        ".##..",
        ".####",
        "#.#.#",
        "#.###",
        "##.#.",
        "##.##",
        "###..",
        "###.#",
        "####.",
    ];
    let pot_map = PotMap::from_iter(
        patterns.iter().map(|x| x.parse::<PotArea>().unwrap())
    );

    let future = simulate(20, pots, &pot_map);
    assert_eq!(future.offset, -2);
    assert_eq!(future.sum(), 325);
}

#[test]
fn test_cycle_shortcut() {
    let state = "##";
    let pots = Pots::from_iter(0, state.chars().map(|x| x == '#'));

    let patterns = vec![
        ".##..",
        "##...",
    ];
    let pot_map = PotMap::from_iter(
        patterns.iter().map(|x| x.parse::<PotArea>().unwrap())
    );

    let future = simulate(50_000_000_000, pots, &pot_map);
    assert_eq!(future.offset, 50_000_000_000);
    assert_eq!(future.sum(), 100_000_000_001);
}
