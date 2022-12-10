use std::collections::HashMap;
use std::fs;

fn build_fs(input: &str) -> HashMap<String, isize> {
    let mut fs: HashMap<String, isize> = HashMap::new();
    fs.insert("/".to_string(), 0);
    let mut filepath = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.strip_prefix("$ cd ").unwrap();
            if dir == ".." {
                filepath.pop();
            } else {
                filepath.push(dir.to_string());
            }
        } else if !line.starts_with("$ ls") {
            if !line.starts_with("dir") {
                let mut splits = line.split(" ");
                let size: isize = splits.next().unwrap().parse().unwrap();
                for i in 0..filepath.len() {
                    *fs.entry(filepath[0..=i].join("/")).or_insert(0) += size;
                }
            }
        }
    }
    fs
}

fn part1(input: &str) {
    let fs = build_fs(input);
    println!(
        "part1: {}",
        fs.iter()
            .map(|(_, v)| *v)
            .filter(|v| *v < 100000)
            .sum::<isize>()
    )
}

fn part2(input: &str) {
    let fs = build_fs(input);
    let total_space = 70000000;
    let desired_space = 30000000;
    let used_space = fs["/"];
    let mut deleted_space = total_space;
    for (_, v) in &fs {
        if used_space - *v <= total_space - desired_space && *v < deleted_space {
            deleted_space = *v;
        }
    }
    println!("part2: {}", deleted_space)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
