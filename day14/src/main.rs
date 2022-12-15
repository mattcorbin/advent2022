use std::cmp::max;
use std::fs;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Square {
    Empty,
    Rock,
    Sand,
    Generator,
}

impl Square {
    fn to_string(&self) -> String {
        match self {
            Square::Empty => ".".to_string(),
            Square::Rock => "#".to_string(),
            Square::Sand => "o".to_string(),
            Square::Generator => "+".to_string(),
        }
    }
}

fn coord_to_tuple(input: &str) -> (usize, usize) {
    let mut splits = input.split(",");
    (
        splits.next().unwrap().parse().unwrap(),
        splits.next().unwrap().parse().unwrap(),
    )
}

fn parse_input(input: &str) -> [[Square; 1000]; 1000] {
    let mut grid = [[Square::Empty; 1000]; 1000];
    for line in input.lines() {
        let mut splits = line.split(" -> ");
        let mut start = coord_to_tuple(splits.next().unwrap());
        loop {
            match splits.next() {
                None => break,
                Some(coord) => {
                    let end = coord_to_tuple(coord);
                    if start.0 != end.0 {
                        if start.0 >= end.0 {
                            for x in end.0..=start.0 {
                                grid[x][start.1] = Square::Rock;
                            }
                        } else {
                            for x in start.0..=end.0 {
                                grid[x][start.1] = Square::Rock;
                            }
                        }
                    } else {
                        if start.1 >= end.1 {
                            for y in end.1..=start.1 {
                                grid[start.0][y] = Square::Rock;
                            }
                        } else {
                            for y in start.1..=end.1 {
                                grid[start.0][y] = Square::Rock;
                            }
                        }
                    }
                    start = end;
                }
            }
        }
    }
    grid[500][0] = Square::Generator;
    grid
}

fn drop_sand(grid: [[Square; 1000]; 1000], lowest_rock: usize) -> [[Square; 1000]; 1000] {
    let mut local = grid.clone();
    let mut current = (500, 0);
    loop {
        if current.1 >= lowest_rock {
            break;
        } else if local[current.0][current.1 + 1] == Square::Empty {
            current = (current.0, current.1 + 1);
        } else if local[current.0 - 1][current.1 + 1] == Square::Empty {
            current = (current.0 - 1, current.1 + 1);
        } else if local[current.0 + 1][current.1 + 1] == Square::Empty {
            current = (current.0 + 1, current.1 + 1);
        } else {
            local[current.0][current.1] = Square::Sand;
            current = (500, 0);
        }
    }
    local
}

fn lowest_rock(grid: [[Square; 1000]; 1000]) -> usize {
    let mut lowest_rock = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] == Square::Rock {
                lowest_rock = max(y, lowest_rock);
            }
        }
    }
    lowest_rock
}

fn count_sand(grid: [[Square; 1000]; 1000]) -> usize {
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] == Square::Sand {
                count += 1;
            }
        }
    }
    count
}

fn part1(input: &str) {
    let mut grid = parse_input(input);
    let mut lowest_rock = lowest_rock(grid);
    println!("part1: {}", count_sand(drop_sand(grid, lowest_rock)))
}

fn drop_sand_floor(grid: [[Square; 1000]; 1000]) -> [[Square; 1000]; 1000] {
    let mut local = grid.clone();
    let mut current = (500, 0);
    loop {
        if local[500][0] == Square::Sand {
            break;
        } else if local[current.0][current.1 + 1] == Square::Empty {
            current = (current.0, current.1 + 1);
        } else if local[current.0 - 1][current.1 + 1] == Square::Empty {
            current = (current.0 - 1, current.1 + 1);
        } else if local[current.0 + 1][current.1 + 1] == Square::Empty {
            current = (current.0 + 1, current.1 + 1);
        } else {
            local[current.0][current.1] = Square::Sand;
            current = (500, 0);
        }
    }
    local
}

fn part2(input: &str) {
    let mut grid = parse_input(input);
    let mut lowest_rock = lowest_rock(grid);
    let floor = lowest_rock + 2;
    for x in 0..1000 {
        grid[x][floor] = Square::Rock
    }
    println!("part2: {}", count_sand(drop_sand_floor(grid)))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
