use std::fs;
use std::collections::HashMap;

fn day_one_part_one(mut left_list: Vec<i32>, mut right_list: Vec<i32>) {

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    for i in 0..left_list.len() {
        total_distance += (left_list[i] - right_list[i]).abs();
    }

    println!("Total distance: {}", total_distance);
    // println!("{:?}", left_list);
    // println!("{:?}", right_list);
}

fn day_one_part_two(left_list: Vec<i32>, right_list: Vec<i32>) {
    let mut hash_map = HashMap::<i32, i32>::new();

    for i in 0..right_list.len() {
        if hash_map.contains_key(&right_list[i]) {
            let val = *hash_map.get(&right_list[i]).unwrap();
            hash_map.insert(right_list[i], val + 1);
        } else {
            hash_map.insert(right_list[i], 1);
        }
    }

    let mut distance = 0;
    for i in 0..left_list.len() {
        if hash_map.contains_key(&left_list[i]) {
            distance += hash_map.get(&left_list[i]).unwrap() * left_list[i];
        }
    }

    println!("Total distance: {}", distance);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|line| line.split("   ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let mut left_list = lines.iter().map(|line| line[0].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut right_list = lines.iter().map(|line| line[1].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // let left_list = vec![3, 4, 2, 1, 3, 3];
    // let right_list = vec![4, 3, 5, 3, 9, 3];

    day_one_part_two(left_list, right_list);
}
