use std::fs;


fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut retval = Vec::new();
    let mut current = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            retval.push(current);
            current = Vec::new();
        } else {
            let calories = line.parse::<usize>().expect("should be a number");
            current.push(calories);
        }
    }
    retval
}

fn part1(input: &str) {
    let elves = parse_input(input);
    let max_calories: usize = elves.iter().map(|x| x.iter().sum()).max().unwrap();
    println!("part1: {}", max_calories)
}

fn part2(input: &str) {
    let elves = parse_input(input);
    let mut elves = elves.iter().map(|x| x.iter().sum()).collect::<Vec<usize>>();
    elves.sort();
    elves.reverse();

    println!("part2: {}", elves[0] + elves[1] + elves[2])
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
