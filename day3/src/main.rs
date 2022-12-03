use std::fs;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut retval = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            retval.push(line.chars().collect::<Vec<char>>());
        }
    }
    retval
}

fn char_to_priority(input: &char) -> usize {
    if input.is_lowercase() {
        (*input) as usize - 96
    } else {
        (*input) as usize - 38
    }
}

fn part1(input: &str) {
    let mut rucksacks = parse_input(input);
    let mut errors = Vec::new();
    for rucksack in rucksacks {
        let compartments: Vec<&[char]> = rucksack.chunks(rucksack.len()/2).collect();
        for item in compartments[0] {
            if compartments[1].contains(item) {
                errors.push(item.clone());
                break
            }
        }
    }
    let sum = errors.iter().map(|x| char_to_priority(x)).sum::<usize>();
    println!("part1: {}", sum)
}

fn part2(input: &str) {
    let mut rucksacks = parse_input(input);
    let mut groups: Vec<&[Vec<char>]> = rucksacks.chunks(3).collect();
    let mut badges = Vec::new();
    for group in groups {
        let mut end = false;
        for a in &group[0] {
            for b in &group[1] {
                for c in &group[2] {
                    if *a == *b && *b == *c {
                        badges.push(a);
                        end = true;
                        break
                    }
                }
                if end {
                    break
                }
            }
            if end {
                break
            }
        }
    }
    let sum = badges.iter().map(|x| char_to_priority(x)).sum::<usize>();
    println!("part2: {}", sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
