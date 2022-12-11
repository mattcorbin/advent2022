use std::fs;

struct Elf {
    start: usize,
    end: usize,
}

impl Elf {
    pub fn contains(&self, other: &Elf) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    pub fn overlaps(&self, other: &Elf) -> bool {
        for i in self.start..=self.end {
            if (other.start..=other.end).contains(&i) {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Vec<Vec<Elf>> {
    let mut retval = Vec::new();
    for line in input.lines() {
        let mut current = Vec::new();
        if !line.is_empty() {
            let mut split = line.split(",");
            let first = split.next().unwrap();
            let mut bounds = first.split("-");
            current.push(Elf {
                start: bounds.next().unwrap().parse().unwrap(),
                end: bounds.next().unwrap().parse().unwrap(),
            });
            let second = split.next().unwrap();
            let mut bounds = second.split("-");
            current.push(Elf {
                start: bounds.next().unwrap().parse().unwrap(),
                end: bounds.next().unwrap().parse().unwrap(),
            });
            retval.push(current);
            current = Vec::new();
        }
    }
    retval
}

fn part1(input: &str) {
    let elves = parse_input(input);
    let mut count = 0;
    for set in elves {
        if set[0].contains(&set[1]) || set[1].contains(&set[0]) {
            count += 1;
        }
    }
    println!("part1: {}", count)
}

fn part2(input: &str) {
    let elves = parse_input(input);
    let mut count = 0;
    for set in elves {
        if set[0].overlaps(&set[1]) {
            count += 1;
        }
    }
    println!("part2: {}", count)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
