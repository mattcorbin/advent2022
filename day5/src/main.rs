use std::fs;

struct Stacks {
    items: Vec<Vec<char>>,
}

struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

fn bootstrap_stacks() -> Stacks {
    Stacks {
        items: vec![], // Redacted
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut retval = Vec::new();
    for line in input.lines() {
        let mut splits = line.strip_prefix("move ").unwrap().split(" from ");
        let count = splits.next().unwrap().parse().unwrap();
        let mut stacks = splits.next().unwrap().split(" to ");
        let src = stacks.next().unwrap().parse().unwrap();
        let dst = stacks.next().unwrap().parse().unwrap();
        retval.push(Instruction { count, src, dst });
    }
    retval
}

fn part1(input: &str) {
    let mut stacks = bootstrap_stacks();
    let instructions = parse_instructions(input);
    for instruction in instructions {
        for _ in 0..instruction.count {
            let mv = stacks.items[instruction.src - 1].pop().unwrap();
            stacks.items[instruction.dst - 1].push(mv);
        }
    }
    let mut output = String::new();
    for mut stack in stacks.items {
        if let Some(top) = stack.pop() {
            output.push(top);
        }
    }
    println!("part1: {}", output)
}

fn part2(input: &str) {
    let mut stacks = bootstrap_stacks();
    let instructions = parse_instructions(input);
    for instruction in instructions {
        let mut temp = Vec::new();
        for _ in 0..instruction.count {
            let mv = stacks.items[instruction.src - 1].pop().unwrap();
            temp.push(mv);
        }
        temp.reverse();
        for item in temp {
            stacks.items[instruction.dst - 1].push(item);
        }
    }
    let mut output = String::new();
    for mut stack in stacks.items {
        if let Some(top) = stack.pop() {
            output.push(top);
        }
    }
    println!("part2: {}", output)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
