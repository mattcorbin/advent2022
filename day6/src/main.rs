use std::fs;

fn check_unique(slice: &[char]) -> bool {
    if slice[0] == slice[1]
        || slice[0] == slice[2]
        || slice[0] == slice[3]
        || slice[1] == slice[2]
        || slice[1] == slice[3]
        || slice[2] == slice[3]
    {
        false
    } else {
        true
    }
}

fn part1(input: &str) {
    let mut marker = 0;
    let chars = input.chars().collect::<Vec<char>>();
    for idx in 0..chars.len() {
        if idx < 3 {
            continue;
        }
        if check_unique(&[chars[idx - 3], chars[idx - 2], chars[idx - 1], chars[idx]]) {
            marker = idx;
            break;
        }
    }
    println!("part1: {}", marker + 1)
}

fn any_dupes(test: &str) -> bool {
    for (idx, c) in test.chars().enumerate() {
        let mut local = test.to_string().chars().collect::<Vec<char>>();
        local.remove(idx);
        if local.contains(&c) {
            return false;
        }
    }
    true
}

fn part2(input: &str) {
    let mut marker = 0;
    for idx in 0..input.len() {
        if idx < 14 {
            continue;
        }
        if any_dupes(&input[idx - 14..idx]) {
            marker = idx;
            break;
        }
    }
    println!("part2: {}", marker)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
