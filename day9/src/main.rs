use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        })
    }
}

struct Move {
    direction: Direction,
    distance: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");
        Ok(Move {
            direction: Direction::from_str(splits.next().unwrap()).unwrap(),
            distance: splits.next().unwrap().parse().unwrap(),
        })
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in input.lines() {
        moves.push(Move::from_str(line).unwrap());
    }
    moves
}

fn move_tail(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let mut new = tail.clone();
    if head.0 != tail.0 && head.1 != tail.1 {
        let dist_x = head.0 - tail.0;
        let dist_y = head.1 - tail.1;
        if dist_x.abs() > 1 || dist_y.abs() > 1 {
            match (dist_x.is_negative(), dist_y.is_negative()) {
                (true, true) => {
                    new.0 -= 1;
                    new.1 -= 1;
                }
                (true, false) => {
                    new.0 -= 1;
                    new.1 += 1;
                }
                (false, true) => {
                    new.0 += 1;
                    new.1 -= 1;
                }
                (false, false) => {
                    new.0 += 1;
                    new.1 += 1;
                }
            }
        }
    } else if head.0 != tail.0 {
        let dist = head.0 - tail.0;
        if dist.abs() > 1 {
            if dist.is_negative() {
                new.0 -= 1;
            } else {
                new.0 += 1;
            }
        }
    } else {
        let dist = head.1 - tail.1;
        if dist.abs() > 1 {
            if dist.is_negative() {
                new.1 -= 1;
            } else {
                new.1 += 1;
            }
        }
    }
    new
}

fn part1(input: &str) {
    let mut visted: HashSet<(isize, isize)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let moves = parse_input(input);
    visted.insert(tail);
    for mv in moves {
        for _ in 0..mv.distance {
            match mv.direction {
                Direction::Up => {
                    head.1 += 1;
                }
                Direction::Down => {
                    head.1 -= 1;
                }
                Direction::Left => {
                    head.0 -= 1;
                }
                Direction::Right => {
                    head.0 += 1;
                }
            }
            tail = move_tail(head, tail);
            visted.insert(tail);
        }
    }
    println!("part1: {}", visted.len())
}

fn part2(input: &str) {
    let mut visted: HashSet<(isize, isize)> = HashSet::new();
    let mut snake = Vec::new();
    for _ in 0..10 {
        snake.push((0, 0));
    }
    let moves = parse_input(input);
    visted.insert(snake[9]);
    for mv in moves {
        for _ in 0..mv.distance {
            match mv.direction {
                Direction::Up => {
                    snake[0] = (snake[0].0, snake[0].1 + 1);
                }
                Direction::Down => {
                    snake[0] = (snake[0].0, snake[0].1 - 1);
                }
                Direction::Left => {
                    snake[0] = (snake[0].0 - 1, snake[0].1);
                }
                Direction::Right => {
                    snake[0] = (snake[0].0 + 1, snake[0].1);
                }
            }
            for i in 1..10 {
                snake[i] = move_tail(snake[i - 1], snake[i]);
            }
            visted.insert(snake[9]);
        }
    }
    println!("part2: {}", visted.len())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
