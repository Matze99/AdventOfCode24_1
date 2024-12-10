use std::fs;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn move_x(&self, i: i32, max_x: usize) -> Result<usize, String> {
        match self {
            Direction::Up | Direction::UpLeft | Direction::UpRight => {
                let mut sub = i - 1;
                if sub < 0 {
                    return Err("Out of bounds".to_string());
                }
                let sub_u = sub as usize;
                Ok(sub_u)
            },
            Direction::Down | Direction::DownLeft | Direction::DownRight => {
                let add = (i + 1) as usize;
                if add >= max_x {
                    return Err("Out of bounds".to_string());
                }
                Ok(add)
            },
            Direction::Left => Ok(i as usize),
            Direction::Right => Ok(i as usize),
        }
    }

    pub fn move_y(&self, i: i32, max_y: usize) -> Result<usize, String> {    
        match self {
            Direction::Left | Direction::UpLeft | Direction::DownLeft => {
                let sub = i - 1;
                if sub < 0 {
                    return Err("Out of bounds".to_string());
                }
                let sub_u = sub as usize;
                Ok(sub_u)
            },
            Direction::Right | Direction::UpRight | Direction::DownRight => {
                let add = (i + 1) as usize;
                if add >= max_y {
                    return Err("Out of bounds".to_string());
                }
                Ok(add)
            },
            Direction::Up | Direction::Down => Ok(i as usize),
        }
    }

    pub fn iter() -> Vec<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::UpLeft, Direction::UpRight, Direction::DownLeft, Direction::DownRight]
    }
}

#[derive(Debug)]
pub enum SearchState {
    X,
    M,
    A,
    S
}

impl SearchState {
    pub fn get_next_char(&self) -> char {
        match self {
            SearchState::X => 'M',
            SearchState::M => 'A',
            SearchState::A => 'S',
            SearchState::S => ' ',
        }
    }
    pub fn next_state(&self) -> SearchState {
        match self {
            SearchState::X => SearchState::M,
            SearchState::M => SearchState::A,
            SearchState::A => SearchState::S,
            SearchState::S => SearchState::X,
        }
    }
}

pub fn search_around(word_matrix: &Vec<&Vec<char>>, i: usize, j: usize, direction: Direction, search_state: SearchState) -> Result<bool, String> {
    // println!("{} {} {} {:?} {:?}", i, j, word_matrix[i][j], search_state, direction);
    let next_char = search_state.get_next_char();
    if next_char == ' ' {
        return Ok(true);
    }
    let next_x = direction.move_x(i as i32, word_matrix.len())?;
    let next_y = direction.move_y(j as i32, word_matrix[i].len())?;
    // println!("next {} {} {}", next_x, next_y, word_matrix[next_x][next_y]);
    if word_matrix[next_x][next_y] == next_char {
        return search_around(word_matrix, next_x, next_y, direction, search_state.next_state());
    }
    Ok(false)
}


pub fn part_one(word_matrix: &Vec<&Vec<char>>) -> i32 {
    let mut counter = 0;
    for i in 0..word_matrix.len() {
        for j in 0..word_matrix[i].len() {
            if word_matrix[i][j] != 'X' {
                continue;
            }
            for direction in Direction::iter() {
                let result = search_around(word_matrix, i, j, direction, SearchState::X);
                if result.is_ok() && result.unwrap() {
                    counter += 1;
                }
            }
        }
    }
    counter 
}

pub fn is_mas(top: char, bottom: char) -> bool {
    match (top, bottom) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    }
}

pub fn part_two(word_matrix: &Vec<&Vec<char>>) -> i32 {
    let mut counter = 0;
    for i in 1..word_matrix.len() - 1 {
        for j in 1..word_matrix[i].len() - 1 {
            if word_matrix[i][j] != 'A' {
                continue;
            }
            let tl = word_matrix[i - 1][j - 1];
            let tr = word_matrix[i - 1][j + 1];
            let bl = word_matrix[i + 1][j - 1];
            let br = word_matrix[i + 1][j + 1];
            if is_mas(tl, br) && is_mas(bl, tr) {
                counter += 1;
            }
        }
    }
    counter 
}


pub fn main() {
    let input = fs::read_to_string("inputs/day_four.txt").unwrap();
    let word_matrix = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let ref_word_matrix = word_matrix.iter().map(|line| line).collect::<Vec<&Vec<char>>>();
    println!("{}", part_two(&ref_word_matrix));
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(Direction::Up, 1, 10, 0)]
    #[case(Direction::Down, 1, 10, 2)]
    #[case(Direction::Left, 1, 10, 1)]
    #[case(Direction::Right, 1, 10, 1)]
    #[case(Direction::UpLeft, 1, 10, 0)]
    #[case(Direction::UpRight, 1, 10, 0)]
    #[case(Direction::DownLeft, 1, 10, 2)]
    #[case(Direction::DownRight, 1, 10, 2)]
    fn test_next_x_success(#[case] direction: Direction, #[case] i: usize, #[case] max_x: usize, #[case] expected: usize) {
        let result = direction.move_x(i as i32, max_x);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(Direction::Up, 1, 10, 1)]
    #[case(Direction::Down, 1, 10, 1)]
    #[case(Direction::Left, 1, 10, 0)]
    #[case(Direction::Right, 1, 10, 2)]
    #[case(Direction::UpLeft, 1, 10, 0)]
    #[case(Direction::UpRight, 1, 10, 2)]
    #[case(Direction::DownLeft, 1, 10, 0)]
    #[case(Direction::DownRight, 1, 10, 2)]
    fn test_next_y_success(#[case] direction: Direction, #[case] i: usize, #[case] max_y: usize, #[case] expected: usize) {
        let result = direction.move_y(i as i32, max_y);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(Direction::Up, 0, 10)]
    #[case(Direction::Down, 9, 10)]
    #[case(Direction::UpLeft, 0, 10)]
    #[case(Direction::UpRight, 0, 10)]
    #[case(Direction::DownLeft, 9, 10)]
    #[case(Direction::DownRight, 9, 10)]
    fn test_next_x_fail(#[case] direction: Direction, #[case] i: usize, #[case] max_x: usize) {
        let result = direction.move_x(i as i32, max_x);
        assert!(result.is_err());
    }

    #[rstest]
    #[case(Direction::Right, 0, 0, SearchState::X)]
    #[case(Direction::Right, 0, 1, SearchState::M)]
    #[case(Direction::Right, 0, 2, SearchState::A)]
    #[case(Direction::Right, 0, 3, SearchState::S)]
    #[case(Direction::Up, 3, 4, SearchState::X)]
    #[case(Direction::Down, 0, 0, SearchState::X)]
    #[case(Direction::DownRight, 0, 0, SearchState::X)]
    fn test_search_around_success(#[case] direction: Direction, #[case] i: usize, #[case] j: usize, #[case] search_state: SearchState) {
        let word_matrix = vec![
            vec!['X', 'M', 'A', 'S', 'S'],
            vec!['M', 'M', 'S', ' ', 'A'],
            vec!['A', 'S', 'A', ' ', 'M'],
            vec!['S', ' ', ' ', 'S', 'X'],
        ];
        let ref_word_matrix = word_matrix.iter().map(|line| line).collect::<Vec<&Vec<char>>>();
        let result = search_around(&ref_word_matrix, i, j, direction, search_state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}