extern crate petgraph;

use std::cmp::min;
use std::collections::HashMap;
use std::fs;

use petgraph::algo::dijkstra;
use petgraph::graphmap::DiGraphMap;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
    height: usize,
}

impl Point {
    fn can_move(&self, other: &Point) -> bool {
        if self.height >= other.height {
            true
        } else if self.height < other.height && other.height - self.height <= 1 {
            true
        } else {
            false
        }
    }
}

fn adjacent_points(x: usize, y: usize) -> Vec<(usize, usize)> {
    let start_x;
    let start_y;

    if x == 0 {
        start_x = 0
    } else {
        start_x = x - 1;
    }

    if y == 0 {
        start_y = 0
    } else {
        start_y = y - 1;
    }

    vec![(start_x, y), (x + 1, y), (x, start_y), (x, y + 1)]
}

fn parse_input(input: &str) -> (Point, Point, Vec<Point>, DiGraphMap<Point, usize>) {
    let mut graph = DiGraphMap::new();
    let mut points = HashMap::new();
    let mut all_points = Vec::new();
    let mut start = Point {
        x: 0,
        y: 0,
        height: 0,
    };
    let mut end = Point {
        x: 0,
        y: 0,
        height: 0,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height;
            let point;
            if c == 'S' {
                height = 0;
                point = Point { x, y, height };
                start = point;
            } else if c == 'E' {
                height = 27;
                point = Point { x, y, height };
                end = point;
            } else {
                height = c as usize - 'a' as usize + 1;
                point = Point { x, y, height };
            }
            points.insert((x, y), point);
            all_points.push(point);
        }
    }
    for (_, point) in &points {
        let nearby = adjacent_points(point.x, point.y);
        for item in nearby {
            if let Some(other) = points.get(&item) {
                if point.can_move(other) {
                    graph.add_edge(point.clone(), other.clone(), 0);
                }
            }
        }
    }
    (start, end, all_points, graph)
}

fn part1(input: &str) {
    let (start, end, _, graph) = parse_input(input);
    let res = dijkstra(&graph, start, None, |_| 1);
    println!("part1: {}", res.get(&end).unwrap())
}

fn part2(input: &str) {
    let (_, end, points, graph) = parse_input(input);
    let mut shortest_possible_path = i32::MAX;
    for point in points {
        if point.height <= 1 {
            let res = dijkstra(&graph, point, None, |_| 1);
            if let Some(dist) = res.get(&end) {
                shortest_possible_path = min(shortest_possible_path, *dist);
            }
        }
    }
    println!("part2: {}", shortest_possible_path);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
