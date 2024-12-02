use std::collections::HashMap;

pub fn part_one(mut left_list: Vec<i32>, mut right_list: Vec<i32>) {

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

pub fn part_two(left_list: Vec<i32>, right_list: Vec<i32>) {
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