extern crate core;

use std::fs;

fn parse_input(input: &str) -> Vec<Vec<String>> {
    let mut retval = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            let mut thing = line.split(" ");
            let turn = vec![
                format!("{}", thing.next().unwrap()),
                format!("{}", thing.next().unwrap()),
            ];
            retval.push(turn);
        }
    }
    retval
}

fn part1(input: &str) {
    let moves = parse_input(input);
    let mut score = 0;
    for m in moves {
        let other = &*m[0];
        let you = &*m[1];
        match other {
            "A" => match you {
                "X" => score += 1 + 3,
                "Y" => score += 2 + 6,
                "Z" => score += 3 + 0,
                _ => panic!("oops"),
            },
            "B" => match &*m[1] {
                "X" => score += 1 + 0,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => panic!("oops"),
            },
            "C" => match &*m[1] {
                "X" => score += 1 + 6,
                "Y" => score += 2 + 0,
                "Z" => score += 3 + 3,
                _ => panic!("oops"),
            },
            _ => panic!("oops"),
        }
    }
    println!("part1: {}", score)
}

fn part2(input: &str) {
    let moves = parse_input(input);
    let mut score = 0;
    for m in moves {
        let other = &*m[0];
        let you = &*m[1];
        match other {
            "A" => match you {
                "X" => score += 3 + 0,
                "Y" => score += 1 + 3,
                "Z" => score += 2 + 6,
                _ => panic!("oops"),
            },
            "B" => match &*m[1] {
                "X" => score += 1 + 0,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => panic!("oops"),
            },
            "C" => match &*m[1] {
                "X" => score += 2 + 0,
                "Y" => score += 3 + 3,
                "Z" => score += 1 + 6,
                _ => panic!("oops"),
            },
            _ => panic!("oops"),
        }
    }
    println!("part2: {}", score)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
