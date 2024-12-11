use std::fs;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod utils;

fn main() {
    day_five::main();
    // day_three::main();
    // let input = fs::read_to_string("input.txt").unwrap();
    // let lines = input.lines().map(|line| line.split("   ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    // let mut left_list = lines.iter().map(|line| line[0].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // let mut right_list = lines.iter().map(|line| line[1].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // // let left_list = vec![3, 4, 2, 1, 3, 3];
    // // let right_list = vec![4, 3, 5, 3, 9, 3];

    // day_one::part_two(left_list, right_list);

    // let test_report = day_two::Report{
    //     levels: vec![7, 6, 4, 2, 1]
    // };
    // let test_report_list = day_two::ReportList{
    //     reports: vec![test_report, 
    //     day_two::Report{levels: vec![1, 3, 6, 7, 9]},
    //     day_two::Report{levels: vec![1, 3, 6, 7, 11]},
    //     day_two::Report{levels: vec![1, 3, 6, 7, 6]},
    //         day_two::Report{levels: vec![1, 3, 6, 6, 9]},
    //         day_two::Report{levels: vec![7, 7, 5]}, 
    //         day_two::Report{levels: vec![9, 7, 6, 2, 1]}, 
    //         day_two::Report{levels: vec![1, 2, 7, 8, 9]}] 
    // };
    // // let result = day_two::count_safe_reports(day_two::read_input());
    // let result = day_two::count_safe_reports_with_dampener(day_two::read_input());
    // println!("Result: {}", result);
}
