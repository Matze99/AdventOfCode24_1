use std::fs;

pub fn read_input(file_path: &str, delimiter: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file_path).unwrap();
    input.lines().map(|line| line.split(delimiter).map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>()
}
