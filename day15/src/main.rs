extern crate rayon;

use std::collections::HashSet;
use std::fs;

use rayon::prelude::*;

#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Sensor {
    location: Point,
    beacon: Point,
    coverage_distance: i64,
}

impl Sensor {
    fn new(location: Point, beacon: Point) -> Sensor {
        let coverage_distance = manhattan(&location, &beacon);
        Sensor {
            location,
            beacon,
            coverage_distance,
        }
    }

    fn from_str(input: &str) -> Sensor {
        let mut splits = input.split(": ");

        let mut sensor_splits = splits
            .next()
            .unwrap()
            .strip_prefix("Sensor at x=")
            .unwrap()
            .split(", y=");
        let sensor_x = sensor_splits.next().unwrap().parse().unwrap();
        let sensor_y = sensor_splits.next().unwrap().parse().unwrap();

        let mut beacon_splits = splits
            .next()
            .unwrap()
            .strip_prefix("closest beacon is at x=")
            .unwrap()
            .split(", y=");
        let beacon_x = beacon_splits.next().unwrap().parse().unwrap();
        let beacon_y = beacon_splits.next().unwrap().parse().unwrap();

        Sensor::new(
            Point {
                x: sensor_x,
                y: sensor_y,
            },
            Point {
                x: beacon_x,
                y: beacon_y,
            },
        )
    }
}

fn manhattan(p: &Point, q: &Point) -> i64 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();
    for line in input.lines() {
        sensors.push(Sensor::from_str(line));
    }
    sensors
}

fn check_blocked(x: i64, sensors: &Vec<Sensor>) -> bool {
    let point = Point { x, y: 2000000 };
    for sensor in sensors {
        if point != sensor.beacon && manhattan(&sensor.location, &point) <= sensor.coverage_distance
        {
            return true;
        }
    }
    false
}

fn part1(input: &str) {
    let sensors = parse_input(input);
    let blocked = (-1000000..=6000000)
        .into_par_iter()
        .map(|x| check_blocked(x, &sensors))
        .filter(|v| *v == true)
        .collect::<Vec<bool>>();
    println!("part1: {}", blocked.len())
}

fn part2(input: &str) {
    let sensors = parse_input(input);
    let mut points_to_test = HashSet::new();
    for sensor in &sensors {
        for i in 0..=(sensor.coverage_distance + 1) {
            points_to_test.insert(Point {
                x: sensor.location.x - sensor.coverage_distance - 1 + i,
                y: sensor.location.y + i,
            });
            points_to_test.insert(Point {
                x: sensor.location.x - sensor.coverage_distance - 1 + i,
                y: sensor.location.y - i,
            });
            points_to_test.insert(Point {
                x: sensor.location.x - sensor.coverage_distance + 1 - i,
                y: sensor.location.y + i,
            });
            points_to_test.insert(Point {
                x: sensor.location.x - sensor.coverage_distance + 1 - i,
                y: sensor.location.y - i,
            });
        }
    }
    for point in points_to_test.clone() {
        if point.x > 4000000 || point.x < 0 || point.y > 4000000 || point.y < 0 {
            points_to_test.remove(&point);
        }
    }
    for sensor in &sensors {
        let local_points = points_to_test.clone();
        for point in local_points {
            if manhattan(&point, &sensor.location) <= sensor.coverage_distance {
                points_to_test.remove(&point);
            }
        }
    }
    let point = points_to_test.into_iter().collect::<Vec<Point>>()[0];
    println!("part2: {}", (point.x as u64 * 4000000) + point.y as u64);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
