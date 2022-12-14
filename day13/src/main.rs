extern crate rayon;
extern crate serde_json;

use std::cmp::min;
use std::fs;

use rayon::prelude::*;

use serde_json::Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Decision {
    Left,
    Right,
    Equal,
}

fn parse_input(input: &str) -> Vec<Vec<Value>> {
    let mut retval = Vec::new();
    let mut local = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            retval.push(local);
            local = Vec::new();
        } else {
            local.push(serde_json::from_str(line).unwrap());
        }
    }
    retval.push(local);
    retval
}

fn compare(lhs: &Value, rhs: &Value) -> Decision {
    if lhs.is_number() && rhs.is_number() {
        let left: usize = lhs.to_string().parse().unwrap();
        let right: usize = rhs.to_string().parse().unwrap();
        return if left == right {
            Decision::Equal
        } else if left < right {
            Decision::Left
        } else {
            Decision::Right
        };
    } else if lhs.is_array() && rhs.is_number() {
        let new_rhs = Value::Array(vec![rhs.clone()]);
        compare(lhs, &new_rhs)
    } else if lhs.is_number() && rhs.is_array() {
        let new_lhs = Value::Array(vec![lhs.clone()]);
        compare(&new_lhs, rhs)
    } else {
        let left = lhs.as_array().unwrap();
        let right = rhs.as_array().unwrap();
        for i in 0..min(left.len(), right.len()) {
            let decision = compare(&left[i], &right[i]);
            if decision != Decision::Equal {
                return decision;
            }
        }
        if left.len() == right.len() {
            Decision::Equal
        } else if left.len() > right.len() {
            Decision::Right
        } else {
            Decision::Left
        }
    }
}

fn part1(input: &str) {
    let pairs = parse_input(input);
    let mut count = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        match compare(&pair[0], &pair[1]) {
            Decision::Left => {
                count += idx + 1;
            }
            Decision::Right => (),
            Decision::Equal => panic!("shouldn't get here"),
        }
    }
    println!("part1: {}", count)
}

fn get_packet_vec(input: &str) -> Vec<Value> {
    let pairs = parse_input(input);
    let mut packets = Vec::new();
    for pair in pairs {
        for packet in pair {
            packets.push(packet);
        }
    }
    packets
}

fn part2(input: &str) {
    let mut packets = get_packet_vec(input);
    let divider_1 = Value::Array(vec![Value::Array(vec![serde_json::from_str("2").unwrap()])]);
    packets.push(divider_1.clone());
    let divider_2 = Value::Array(vec![Value::Array(vec![serde_json::from_str("6").unwrap()])]);
    packets.push(divider_2.clone());
    let mut ordered_packets = Vec::with_capacity(packets.len());
    while packets.len() > 0 {
        for i in 0..packets.len() {
            let item = &packets[i];
            let res = packets
                .par_iter()
                .filter(|x| *x != item)
                .map(|x| compare(item, x))
                .filter(|x| *x != Decision::Left)
                .collect::<Vec<Decision>>();
            if res.len() == 0 {
                ordered_packets.push(item.clone());
                packets.remove(i);
                break;
            }
        }
    }
    let mut idx1 = 0;
    let mut idx2 = 0;
    for (idx, packet) in ordered_packets.iter().enumerate() {
        if *packet == divider_1 {
            idx1 = idx + 1;
        } else if *packet == divider_2 {
            idx2 = idx + 1;
        }
    }

    println!("part2: {}", idx1 * idx2)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
