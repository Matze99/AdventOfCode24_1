use std::fs;

mod day_one;
mod day_two;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|line| line.split("   ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let mut left_list = lines.iter().map(|line| line[0].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut right_list = lines.iter().map(|line| line[1].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // let left_list = vec![3, 4, 2, 1, 3, 3];
    // let right_list = vec![4, 3, 5, 3, 9, 3];

    day_one::part_two(left_list, right_list);
}
