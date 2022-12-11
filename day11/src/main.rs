use std::fs;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: fn(u128) -> u128,
    test: fn(u128) -> usize,
}

fn build_example_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            operation: |x| x * 19,
            test: |x| if x % 23 == 0 { 2 } else { 3 },
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |x| x + 6,
            test: |x| if x % 19 == 0 { 2 } else { 0 },
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |x| x * x,
            test: |x| if x % 13 == 0 { 1 } else { 3 },
        },
        Monkey {
            items: vec![74],
            operation: |x| x + 3,
            test: |x| if x % 17 == 0 { 0 } else { 1 },
        },
    ]
}

fn build_monkeys() -> Vec<Monkey> {
    vec![
        // REDACTED
    ]
}

fn part1(_: &str) {
    let mut monkeys = build_example_monkeys();
    let mut inspections = Vec::new();
    for _ in 0..monkeys.len() {
        inspections.push(0);
    }
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items = Vec::new();
            for item in items {
                let new_item = (monkeys[i].operation)(item);
                let new_item = new_item / 3;
                let next_monkey = (monkeys[i].test)(new_item);
                monkeys[next_monkey].items.push(new_item);
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    println!("part1: {}", inspections[0] * inspections[1])
}

fn part2(_: &str) {
    let mut monkeys = build_monkeys();
    let mut inspections: Vec<u128> = Vec::new();
    for _ in 0..monkeys.len() {
        inspections.push(0);
    }
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items = Vec::new();
            for item in items {
                let new_item = (monkeys[i].operation)(item);
                let new_item = new_item % 9699690; // 96577 for example
                let next_monkey = (monkeys[i].test)(new_item);
                monkeys[next_monkey].items.push(new_item);
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    println!("part2: {}", inspections[0] * inspections[1])
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
