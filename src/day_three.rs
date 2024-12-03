use std::fs;
use regex::Regex;


pub fn part_one(line: String) -> i64 {

    let r = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches = r.captures_iter(&line);
    let mut result: i64 = 0;
    matches.for_each(|m| {
        let m = m.get(0).unwrap().as_str().to_string();
        let length = m.len();
        let nums = m[4..length-1].split(",").collect::<Vec<&str>>();

        let a = nums[0].parse::<i64>().unwrap();
        let b = nums[1].parse::<i64>().unwrap();

        result += a * b;
    });

    result
}

pub fn part_two(line: String) {

    let r = Regex::new(r"(do\(\))|(don\'t\(\))").unwrap();
    let matches = r.captures_iter(&line);
    let mut result: i64 = 0;
    let mut prev_do = true;
    let mut prev_start = 0;
    matches.for_each(|m| {
        let m_unwrap = m.get(0).unwrap();   

        let start = m_unwrap.start();
        if prev_do {
            let prev_str = &line[prev_start..start];
            result += part_one(prev_str.to_string());
        }

        let m = m_unwrap.as_str().to_string();
        if m == "do()" {
            prev_do = true;
        } else {
            prev_do = false;
        }
        prev_start = m_unwrap.end();
    });

    if prev_do {
        let prev_str = &line[prev_start..];
        result += part_one(prev_str.to_string());
    }

    println!("{}", result);
}

pub fn main() {
    
    let input = fs::read_to_string("./inputs/day_three.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let line = lines.iter().fold(String::new(), |acc, line| acc + line);

    println!("{}", part_one(line.clone()));
    part_two(line);
}