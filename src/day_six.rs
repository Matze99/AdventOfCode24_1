use std::fs;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum TileState {
    Empty,
    Wall,
}

struct Tile {
    state: TileState,
    visited: bool,
}


type Map = Vec<Vec<Tile>>;

struct Guard {
    position: (i32, i32),
    direction: Direction,
    max_x: i32,
    max_y: i32,
}

impl Guard {
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }

    fn get_next_position(&self) -> Result<(i32, i32), String> {
        // check out of bounds
        match (self.direction, self.position.0, self.position.1) {
            (Direction::Up, _, 0) | (Direction::Left, 0, _) => {
                return Err("Out of bounds".to_string());
            }
            (Direction::Right, t, _) => {
                if t == self.max_x - 1 {
                    return Err("Out of bounds".to_string());
                }
            }
            (Direction::Down, _, t) => {
                if t == self.max_y - 1 {
                    return Err("Out of bounds".to_string());
                }
            }
            _ => {},
        }
        match self.direction {
            Direction::Up => Ok((self.position.0, self.position.1 - 1)),
            Direction::Down => Ok((self.position.0, self.position.1 + 1)),
            Direction::Left => Ok((self.position.0 - 1, self.position.1)),
            Direction::Right => Ok((self.position.0 + 1, self.position.1)),
        }
    }
}


pub fn part_one(mut map: Map, mut guard: Guard) {

}

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut map: Map = Vec::new();
    let mut start = (0, 0);
    let mut direction = Direction::Up;

    for (i, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (j, char) in line.chars().enumerate() {
            match char {
                '.' => row.push(Tile { state: TileState::Empty, visited: false }),
                '#' => row.push(Tile { state: TileState::Wall, visited: false }),
                '^' => {
                    row.push(Tile { state: TileState::Empty, visited: true });
                    start = (i, j);
                    direction = Direction::Up;
                }
                'v' => {
                    row.push(Tile { state: TileState::Empty, visited: true });
                    start = (i, j);
                    direction = Direction::Down;
                }
                '<' => {
                    row.push(Tile { state: TileState::Empty, visited: true });
                    start = (i, j);
                    direction = Direction::Left;
                }
                '>' => {
                    row.push(Tile { state: TileState::Empty, visited: true });
                    start = (i, j);
                    direction = Direction::Right;
                }
                _ => panic!("Invalid character: {}", char),
            }
        }
        map.push(row);
    }

    let mut guard = Guard { position: start, direction, max_x: map.len() as i32, max_y: map[0].len() as i32 };

    let lines = input.lines().map(|line| line.split("   ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
}
