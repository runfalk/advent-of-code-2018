use aoc::{buf_reader_from_arg, parse_lines};
use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Registers {
    values: [usize; 4],
}

impl Registers {
    fn from_values(a: usize, b: usize, c: usize, d: usize) -> Self {
        Self { values: [a, b, c, d] }
    }
}

impl Index<usize> for Registers {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        &self.values[i]
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, i: usize) -> &mut usize {
        &mut self.values[i]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
use self::OpCode::*;

impl OpCode {
    fn eval(&self, registers: &mut Registers, a: usize, b: usize, c: usize) {
        match *self {
            Addr => {
                registers[c] = registers[a] + registers[b];
            },
            Addi => {
                registers[c] = registers[a] + b;
            },
            Mulr => {
                registers[c] = registers[a] * registers[b];
            },
            Muli => {
                registers[c] = registers[a] * b;
            },
            Banr => {
                registers[c] = registers[a] & registers[b];
            },
            Bani => {
                registers[c] = registers[a] & b;
            },
            Borr => {
                registers[c] = registers[a] | registers[b];
            },
            Bori => {
                registers[c] = registers[a] | b;
            },
            Setr => {
                registers[c] = registers[a];
            },
            Seti => {
                registers[c] = a;
            },
            Gtir => {
                registers[c] = (a > registers[b]) as usize;
            },
            Gtri => {
                registers[c] = (registers[a] > b) as usize;
            },
            Gtrr => {
                registers[c] = (registers[a] > registers[b]) as usize;
            },
            Eqir => {
                registers[c] = (a == registers[b]) as usize;
            },
            Eqri => {
                registers[c] = (registers[a] == b) as usize;
            },
            Eqrr => {
                registers[c] = (registers[a] == registers[b]) as usize;
            },
        }
    }

    fn values() -> Iter<'static, Self> {
        static OPCODES: [OpCode; 16] = [
            Addr,
            Addi,
            Mulr,
            Muli,
            Banr,
            Bani,
            Borr,
            Bori,
            Setr,
            Seti,
            Gtir,
            Gtri,
            Gtrr,
            Eqir,
            Eqri,
            Eqrr,
        ];
        OPCODES.into_iter()
    }
}

fn eval_opcode(mut regs: Registers, opcode: OpCode, a: usize, b: usize, c: usize) -> Registers {
    opcode.eval(&mut regs, a, b, c);
    regs
}

fn part_a(tests: impl Iterator<Item = (Registers, (usize, usize, usize, usize), Registers)>) -> usize {
    let mut num_triples = 0;
    for (in_, args, out) in tests {
        let mut num_matching_opcodes = 0;
        for opcode in OpCode::values() {
            let evaled_regs = eval_opcode(in_.clone(), *opcode, args.1, args.2, args.3);
            if evaled_regs == out {
                num_matching_opcodes += 1;
            }
        }
        if num_matching_opcodes >= 3 {
            num_triples += 1;
        }
    }
    num_triples
}

fn part_b(tests: impl Iterator<Item = (Registers, (usize, usize, usize, usize), Registers)>, program: impl Iterator<Item = (usize, usize, usize, usize)>) -> usize {
    let opcodes: HashSet<_> = OpCode::values().map(|x| x.clone()).collect();

    let mut op_map: BTreeMap<usize, HashSet<OpCode>> = BTreeMap::new();
    for i in 0..16 {
        op_map.insert(i, opcodes.clone());
    }

    for (in_, args, out) in tests {
        for opcode in OpCode::values() {
            let evaled_regs = eval_opcode(in_.clone(), *opcode, args.1, args.2, args.3);
            if evaled_regs != out {
                op_map.get_mut(&args.0).unwrap().remove(opcode);
            }
        }
    }

    let mut done_opcodes: HashSet<OpCode> = HashSet::new();
    while done_opcodes != opcodes {
        done_opcodes.extend(op_map
            .values()
            .filter(|x| x.len() == 1)
            .map(|x| x.iter().next().unwrap())
        );

        for (code, possible_ops) in op_map.range_mut(..) {
            if possible_ops.len() == 1 {
                continue;
            }
            for done_op in done_opcodes.iter() {
                possible_ops.remove(&done_op);
            }
        }
    }

    // Convert the opcodes to a true map
    let opcodes: HashMap<usize, OpCode> = op_map.into_iter().map(|(k, v)| (k, v.iter().next().unwrap().clone())).collect();
    println!("{:#?}", opcodes);

    let mut regs = Registers::from_values(0, 0, 0, 0);
    for (op, a, b, c) in program {
        opcodes[&op].eval(&mut regs, a, b, c);
    }
    regs[0]
}

fn main() {
    let reg_re = Regex::new(r"^(?:Before:|After: ) \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
    let op_re = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();
    let mut tests = Vec::new();
    let mut lines = parse_lines::<String>(buf_reader_from_arg().unwrap());

    // Parse the samples from the first half of the file
    loop {
        let in_line = lines.next().unwrap();
        if in_line == "" {
            break;
        }
        let op_line = lines.next().unwrap();
        let out_line = lines.next().unwrap();

        // Skip blank line
        lines.next().unwrap();

        let in_capt = reg_re.captures(&in_line).unwrap();
        let in_ = Registers::from_values(
            in_capt[1].parse::<usize>().unwrap(),
            in_capt[2].parse::<usize>().unwrap(),
            in_capt[3].parse::<usize>().unwrap(),
            in_capt[4].parse::<usize>().unwrap(),
        );

        let op_capt = op_re.captures(&op_line).unwrap();
        let op = (
            op_capt[1].parse::<usize>().unwrap(),
            op_capt[2].parse::<usize>().unwrap(),
            op_capt[3].parse::<usize>().unwrap(),
            op_capt[4].parse::<usize>().unwrap(),
        );

        let out_capt = reg_re.captures(&out_line).unwrap();
        let out = Registers::from_values(
            out_capt[1].parse::<usize>().unwrap(),
            out_capt[2].parse::<usize>().unwrap(),
            out_capt[3].parse::<usize>().unwrap(),
            out_capt[4].parse::<usize>().unwrap(),
        );

        tests.push((in_, op, out));
    }

    // Skip remaining blank line before program input starts
    lines.next();

    let program = lines.map(move |x| {
        let op_capt = op_re.captures(&x).unwrap();
        (
            op_capt[1].parse::<usize>().unwrap(),
            op_capt[2].parse::<usize>().unwrap(),
            op_capt[3].parse::<usize>().unwrap(),
            op_capt[4].parse::<usize>().unwrap(),
        )
    });

    println!("Answer A: {}", part_a(tests.clone().into_iter()));
    println!("Answer B: {}", part_b(tests.into_iter(), program));
}

#[test]
fn test_opcode_eval() {
    assert_eq!(eval_opcode(Registers::from_values(3, 2, 1, 1), Mulr, 2, 1, 2), Registers::from_values(3, 2, 2, 1));
    assert_eq!(eval_opcode(Registers::from_values(3, 2, 1, 1), Addi, 2, 1, 2), Registers::from_values(3, 2, 2, 1));
    assert_eq!(eval_opcode(Registers::from_values(3, 2, 1, 1), Seti, 2, 1, 2), Registers::from_values(3, 2, 2, 1));
}
