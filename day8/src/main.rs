use std::cmp::max;
use std::fs;

fn parse_input(input: &str) -> [[u32; 99]; 99] {
    let mut grid: [[u32; 99]; 99] = [[0; 99]; 99];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c.to_digit(10) {
                Some(digit) => grid[x][y] = digit,
                None => println!("{}", c),
            }
        }
    }
    grid
}

fn check_visible(grid: &[[u32; 99]; 99], x: usize, y: usize) -> bool {
    if x == 0 || x == 99 || y == 0 || y == 99 {
        return true;
    }
    let height = grid[x][y];
    let mut vis_left = true;
    let mut vis_right = true;
    let mut vis_up = true;
    let mut vis_down = true;
    for new_x in 0..x {
        if grid[new_x][y] >= height {
            vis_left = false;
            break;
        }
    }
    for new_x in x + 1..99 {
        if grid[new_x][y] >= height {
            vis_right = false;
            break;
        }
    }
    for new_y in 0..y {
        if grid[x][new_y] >= height {
            vis_down = false;
            break;
        }
    }
    for new_y in y + 1..99 {
        if grid[x][new_y] >= height {
            vis_up = false;
            break;
        }
    }
    vis_left || vis_right || vis_up || vis_down
}

fn part1(input: &str) {
    let grid = parse_input(input);
    let mut visible = 0;
    for x in 0..99 {
        for y in 0..99 {
            if check_visible(&grid, x, y) {
                visible += 1;
            }
        }
    }
    println!("part1: {}", visible)
}

fn check_score(grid: &[[u32; 99]; 99], x: usize, y: usize) -> usize {
    let height = grid[x][y];
    let mut dist_left = 0;
    let mut dist_right = 0;
    let mut dist_up = 0;
    let mut dist_down = 0;
    for new_x in (0..x).rev() {
        if grid[new_x][y] >= height {
            dist_left = x - new_x;
            break;
        } else if new_x == 0 {
            dist_left = x - new_x;
        }
    }
    for new_x in (x + 1)..99 {
        if grid[new_x][y] >= height {
            dist_right = new_x - x;
            break;
        } else if new_x == 98 {
            dist_right = new_x - x;
        }
    }
    for new_y in (0..y).rev() {
        if grid[x][new_y] >= height {
            dist_down = y - new_y;
            break;
        } else if new_y == 0 {
            dist_down = y - new_y;
        }
    }
    for new_y in (y + 1)..99 {
        if grid[x][new_y] >= height {
            dist_up = new_y - y;
            break;
        } else if new_y == 98 {
            dist_up = new_y - y;
        }
    }
    dist_left * dist_right * dist_up * dist_down
}

fn part2(input: &str) {
    let grid = parse_input(input);
    let mut highest_score = 0;
    for x in 0..99 {
        for y in 0..99 {
            highest_score = max(highest_score, check_score(&grid, x, y));
        }
    }
    println!("part2: {}", highest_score)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
