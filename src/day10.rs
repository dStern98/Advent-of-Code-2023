use std::collections::HashSet;

use crate::{read_file_to_string, SolveAdvent};

pub struct Day10;

impl SolveAdvent for Day10 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let pipe_map = PipeMap::new(&file_as_str);
        println!("Location of S: {:?}", pipe_map.start);
        let (pipe_explorer1, pipe_explorer2) = pipe_map.find_two_pipes_connected_to_s();
        let furthest_distance = find_largest_distance_from_s(pipe_explorer1, pipe_explorer2);
        println!(
            "The furthest distance traversal through the pipe is {}",
            furthest_distance
        );
    }

    fn solve_part2(path_to_file: &str) {
        let _ = read_file_to_string(path_to_file);
    }
}

#[derive(Debug, Clone)]
struct PipeMap {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

impl PipeMap {
    fn find_s_position(map: &[Vec<char>]) -> (usize, usize) {
        //! Self-explanatory. Find the row,col position of S
        for (row_number, row) in map.iter().enumerate() {
            for (col_number, symbol) in row.iter().enumerate() {
                if symbol == &'S' {
                    return (row_number, col_number);
                }
            }
        }
        panic!("Map did not contain an S");
    }
    fn new(file_as_str: &str) -> PipeMap {
        //! Build a new `PipeMap` from the input file.
        let map = file_as_str
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let s_position = PipeMap::find_s_position(&map);
        PipeMap {
            map,
            start: s_position,
        }
    }

    fn get_pipe_value(&self, row: usize, col: usize) -> Option<&char> {
        //! Return the value in the map corresponding to the passed in row, col
        //! if it exists
        if let Some(row) = self.map.get(row) {
            if let Some(pipe_symbol) = row.get(col) {
                return Some(pipe_symbol);
            }
        }
        None
    }

    fn find_two_pipes_connected_to_s(&self) -> (PipeExplorer<'_>, PipeExplorer<'_>) {
        //! Start from the position of S, look up, down, left, right.
        //! The prompt gurantees that exactly 2 pipes connected to S form the pipe
        //! loop.
        let mut valid_starting_positions = Vec::new();
        let valid_pipes_above = ['|', '7', 'F'];
        let valid_pipes_below = ['|', 'L', 'J'];
        let valid_pipes_left = ['L', 'F', '-'];
        let valid_pipes_right = ['7', 'J', '-'];

        if self.start.0 > 0 {
            let (new_row, new_col) = (self.start.0 - 1, self.start.1);
            if let Some(pipe_symbol) = self.get_pipe_value(new_row, new_col) {
                if valid_pipes_above.contains(pipe_symbol) {
                    valid_starting_positions.push(PipeExplorer {
                        row: new_row,
                        col: new_col,
                        steps: 1,
                        direction: Direction::Up,
                        pipe_map: self,
                    });
                }
            }
        }
        if self.start.1 > 0 {
            let (new_row, new_col) = (self.start.0, self.start.1 - 1);
            if let Some(pipe_symbol) = self.get_pipe_value(new_row, new_col) {
                if valid_pipes_left.contains(pipe_symbol) {
                    valid_starting_positions.push(PipeExplorer {
                        row: new_row,
                        col: new_col,
                        steps: 1,
                        direction: Direction::Left,
                        pipe_map: self,
                    });
                }
            }
        }
        if let Some(pipe_symbol) = self.get_pipe_value(self.start.0 + 1, self.start.1) {
            if valid_pipes_below.contains(pipe_symbol) {
                valid_starting_positions.push(PipeExplorer {
                    row: self.start.0 + 1,
                    col: self.start.1,
                    steps: 1,
                    direction: Direction::Down,
                    pipe_map: self,
                });
            }
        }
        if let Some(pipe_symbol) = self.get_pipe_value(self.start.0, self.start.1 + 1) {
            if valid_pipes_right.contains(pipe_symbol) {
                valid_starting_positions.push(PipeExplorer {
                    row: self.start.0,
                    col: self.start.1 + 1,
                    steps: 1,
                    direction: Direction::Right,
                    pipe_map: self,
                });
            }
        }
        if valid_starting_positions.len() != 2 {
            panic!("Could not find exactly two pipe connected to S");
        }

        (
            valid_starting_positions.first().unwrap().clone(),
            valid_starting_positions.get(1).unwrap().clone(),
        )
    }
}

///The four possible directions an explorer
/// can be moving.
#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

///A probe that travels the pipe loop.
#[derive(Debug, Clone)]
struct PipeExplorer<'a> {
    ///the number of steps the explorer has traveled
    steps: usize,
    /// the explorers current row position
    row: usize,
    /// the explorers current column position
    col: usize,
    /// the explorers current direction of movement.
    direction: Direction,
    ///A reference to the map that the probe is traveling.
    pipe_map: &'a PipeMap,
}

impl<'a> PipeExplorer<'a> {
    fn collided(&self, other: &PipeExplorer) -> bool {
        self.row == other.row && self.col == other.col
    }
    fn get_new_position(&self, new_direction: &Direction) -> (usize, usize) {
        match new_direction {
            Direction::Down => (self.row + 1, self.col),
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Left => (self.row, self.col - 1),
        }
    }

    fn next_move(&mut self) {
        //! Following the traversal rules of the Day10 problem,
        //! move the explorer to the next pipe.
        let current_pipe_symbol = self
            .pipe_map
            .get_pipe_value(self.row, self.col)
            .expect("PipeExplorer went off the map!");
        let new_direction = match self.direction {
            //New Moves if currently moving down
            Direction::Down if current_pipe_symbol == &'|' => Direction::Down,
            Direction::Down if current_pipe_symbol == &'J' => Direction::Left,
            Direction::Down if current_pipe_symbol == &'L' => Direction::Right,
            //If moving Up
            Direction::Up if current_pipe_symbol == &'|' => Direction::Up,
            Direction::Up if current_pipe_symbol == &'7' => Direction::Left,
            Direction::Up if current_pipe_symbol == &'F' => Direction::Right,
            //If moving Right
            Direction::Right if current_pipe_symbol == &'J' => Direction::Up,
            Direction::Right if current_pipe_symbol == &'7' => Direction::Down,
            Direction::Right if current_pipe_symbol == &'-' => Direction::Right,
            //If move left
            Direction::Left if current_pipe_symbol == &'-' => Direction::Left,
            Direction::Left if current_pipe_symbol == &'L' => Direction::Up,
            Direction::Left if current_pipe_symbol == &'F' => Direction::Down,

            _ => panic!("Illegal movement state!"),
        };
        let (new_row, new_col) = self.get_new_position(&new_direction);
        self.row = new_row;
        self.col = new_col;
        self.direction = new_direction;
        self.steps += 1;
    }
}

fn find_largest_distance_from_s(mut pipe_1: PipeExplorer, mut pipe_2: PipeExplorer) -> usize {
    //! Given two pipe explorers, which the caller must gurantee are the two pipes connected to S,
    //! will return the largest possible distance from S traveling the loop.
    loop {
        pipe_1.next_move();
        if pipe_1.collided(&pipe_2) {
            break;
        };
        pipe_2.next_move();
        if pipe_1.collided(&pipe_2) {
            break;
        }
    }
    pipe_1.steps.max(pipe_2.steps)
}

#[allow(dead_code)]
fn find_all_pipe_locations(
    s_position: (usize, usize),
    mut pipe1: PipeExplorer,
    pipe2: PipeExplorer,
) -> HashSet<(usize, usize)> {
    //! Returns a set of all pipe locations.
    let mut pipe_positions = HashSet::new();
    pipe_positions.insert(s_position);
    while !pipe1.collided(&pipe2) {
        pipe_positions.insert((pipe1.row, pipe1.col));
        pipe1.next_move();
    }
    pipe_positions.insert((pipe2.row, pipe2.col));
    println!("{:?}", pipe_positions);
    pipe_positions
}
