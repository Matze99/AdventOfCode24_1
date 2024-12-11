use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

struct Rule {
    pub left: i32,
    pub right: i32,
}

pub fn set_is_valid(rules: &HashMap<i32, HashSet<i32>>, print: &Vec<i32>) -> bool {
    for num in 0..print.len()-1 {
        if rules.contains_key(&print[num]) {
            if rules.get(&print[num]).unwrap().contains(&print[num+1]) {
                return false;
            }
        }
    }
    true
}

pub fn re_order(print: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut new_print = print.clone();
    for num in 1..print.len() {
        if rules.contains_key(&new_print[num-1]) {
            if rules.get(&new_print[num-1]).unwrap().contains(&new_print[num]) {
                let temp = new_print[num-1];
                new_print[num-1] = new_print[num];
                new_print[num] = temp;
            }
        }
    }
    if set_is_valid(rules, &new_print) {
        return new_print;
    }
    re_order(&new_print, rules)
}

pub fn part_one(rules: &HashMap<i32, HashSet<i32>>, print_list: &Vec<Vec<i32>>) -> i32 {
    let mut counter = 0;
    for print in print_list {
        if set_is_valid(rules, print) {
            if print.len() % 2 == 0 {
                println!("even {:?}", print);
            }
            let middle = print.len() / 2;
            counter += print[middle];
        }
    }
    counter
}

pub fn part_two(rules: &HashMap<i32, HashSet<i32>>, print_list: &Vec<Vec<i32>>) -> i32 {
    let mut counter = 0;
    for print in print_list {
        if set_is_valid(rules, print) {
            // if print.len() % 2 == 0 {
            //     println!("even {:?}", print);
            // }
            // let middle = print.len() / 2;
            // counter += print[middle];
        }
        else {
            let new_set = re_order(print, rules);
            counter += new_set[new_set.len() / 2];
        }
    }
    counter
}

pub fn main() {
    let input = fs::read_to_string("inputs/day_five.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();	
    let mut pages: Vec<Vec<i32>> = vec![];
    let mut is_rule = true;
    for line in lines {
        if line.is_empty() {
            is_rule = false;
            continue;
        }
        if is_rule {
            let splits = line.split("|").collect::<Vec<&str>>();
            let left = splits[0].parse::<i32>().unwrap();
            let right = splits[1].parse::<i32>().unwrap();
            rules.entry(right).or_insert(HashSet::new()).insert(left);
        } else {
            let splits = line.split(",").map(|split| split.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            pages.push(splits);
        }
    }
    println!("{}", part_two(&rules, &pages));
}
