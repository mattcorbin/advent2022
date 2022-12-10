use std::fs;

#[derive(PartialEq)]
enum Operation {
    Noop,
    Addx
}

impl Operation {
    fn from(s: &str) -> Operation {
        match s {
            "noop" => Operation::Noop,
            "addx" => Operation::Addx,
            _ => panic!("invalid operation")
        }
    }
}

struct Instruction {
    op: Operation,
    val: isize,
}

impl Instruction {
    fn from(s: &str) -> Instruction {
        let mut splits = s.split(" ");
        let mut val = 0;
        let op = Operation::from(splits.next().unwrap());
        if op == Operation::Addx {
            val = splits.next().unwrap().parse().unwrap();
        }
        Instruction {
            op,
            val
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::from(line));
    }
    instructions
}

fn part1(input: &str) {
    let instructions = parse_input(input);
    let mut pc = 1;
    let mut delayed = 0;
    let mut val = 1;
    let breakpoints = [20, 60, 100, 140, 180, 220];
    let mut values = Vec::new();
    let mut iter = instructions.iter();
    while true {
        pc += 1;
        if delayed != 0 {
            val += delayed;
            delayed = 0;
        } else {
            match iter.next() {
                None => break,
                Some(instruction) => {
                    if instruction.op == Operation::Addx {
                        delayed = instruction.val;
                    }
                }
            }
        }
        if breakpoints.contains(&pc) {
            println!("{},{}", pc, val);
            values.push(pc*val);
        }
    }
    println!("part1: {}", values.iter().sum::<isize>())
}

fn part2(input: &str) {
    let instructions = parse_input(input);
    let mut pc = 1;
    let mut delayed = 0;
    let mut val = 1;
    let mut iter = instructions.iter();
    print!("#");
    while true {
        let pos = pc%40;
        if pos == 0 {
            print!("\n")
        }
        pc += 1;
        if delayed != 0 {
            val += delayed;
            delayed = 0;
        } else {
            match iter.next() {
                None => break,
                Some(instruction) => {
                    if instruction.op == Operation::Addx {
                        delayed = instruction.val;
                    }
                }
            }
        }
        if [val-1, val, val+1].contains(&pos) {
            print!("#");
        } else {
            print!(".")
        }

    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
