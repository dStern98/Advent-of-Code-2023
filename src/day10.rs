use crate::{read_file_to_string, SolveAdvent};
use std::collections::HashSet;

pub struct Day10;

impl SolveAdvent for Day10 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let pipe_map = PipeMap::new(&file_as_str);
        let (pipe_explorer1, pipe_explorer2) = pipe_map.find_two_pipes_connected_to_s();
        let furthest_distance = find_largest_distance_from_s(pipe_explorer1, pipe_explorer2);
        println!(
            "The furthest distance traversal through the pipe is {}",
            furthest_distance
        );
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let pipe_map = PipeMap::new(&file_as_str);
        let (pipe_explorer1, pipe_explorer2) = pipe_map.find_two_pipes_connected_to_s();
        let known_pipe_locations =
            gather_pipe_locations(pipe_explorer1.clone(), pipe_explorer2.clone());

        //Try all 4 possibilities, as it is not trivial to figure out
        //which of the two S-connected pipes is clockwise and which is counterclockwise.
        compute_surrounded_points(
            pipe_explorer1.clone(),
            &known_pipe_locations,
            LoopDirection::Clockwise,
        );
        compute_surrounded_points(
            pipe_explorer2.clone(),
            &known_pipe_locations,
            LoopDirection::Clockwise,
        );
        compute_surrounded_points(
            pipe_explorer1,
            &known_pipe_locations,
            LoopDirection::Counterclockwise,
        );
        compute_surrounded_points(
            pipe_explorer2,
            &known_pipe_locations,
            LoopDirection::Counterclockwise,
        );
    }
}

///Stores the Input Map and
/// the position of S as a row, col
#[derive(Debug, Clone)]
struct PipeMap {
    map: Vec<Vec<char>>,
    s_position: (usize, usize),
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
        PipeMap { map, s_position }
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

    fn find_two_pipes_connected_to_s(&self) -> (PipeExplorer, PipeExplorer) {
        //! Start from the position of S, look up, down, left, right.
        //! The prompt gurantees that exactly 2 pipes connected to S form the pipe
        //! loop.
        let mut valid_starting_positions = Vec::new();
        let valid_pipes_above = ['|', '7', 'F'];
        let valid_pipes_below = ['|', 'L', 'J'];
        let valid_pipes_left = ['L', 'F', '-'];
        let valid_pipes_right = ['7', 'J', '-'];

        if self.s_position.0 > 0 {
            let (new_row, new_col) = (self.s_position.0 - 1, self.s_position.1);
            if let Some(pipe_symbol) = self.get_pipe_value(new_row, new_col) {
                //If S connect above to a valid pipe for above: '|', '7', 'F', then that is one
                //of the valid pipes connected to S.
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
        if let Some(pipe_symbol) = self.get_pipe_value(self.s_position.0, self.s_position.1 + 1) {
            if valid_pipes_right.contains(pipe_symbol) {
                valid_starting_positions.push(PipeExplorer {
                    row: self.s_position.0,
                    col: self.s_position.1 + 1,
                    steps: 1,
                    direction: Direction::Right,
                    pipe_map: self,
                });
            }
        }
        if self.s_position.1 > 0 {
            let (new_row, new_col) = (self.s_position.0, self.s_position.1 - 1);
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
        if let Some(pipe_symbol) = self.get_pipe_value(self.s_position.0 + 1, self.s_position.1) {
            if valid_pipes_below.contains(pipe_symbol) {
                valid_starting_positions.push(PipeExplorer {
                    row: self.s_position.0 + 1,
                    col: self.s_position.1,
                    steps: 1,
                    direction: Direction::Down,
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
        //! Whether two pipe explorers have collided.
        self.row == other.row && self.col == other.col
    }
    fn move_in_direction(&self, new_direction: &Direction) -> (usize, usize) {
        //! Move a step in the direction specified by new direction.
        match new_direction {
            Direction::Down => (self.row + 1, self.col),
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Left => (self.row, self.col - 1),
        }
    }
    fn get_left_orthogonal(&self) -> Option<(usize, usize)> {
        //! Given an Explorer traversing the loop, return the point directly
        //! left orthogonal to the direction of motion of the pipe.
        //!
        //! This could be impossible
        //! if the wrong Explorer has been chosen for the given loop traversal direction. For example,
        //! if Explorer1 is the true Clockwise explorer but Explorer2 is attempted, then it is possible
        //! that the left orthoginal goes off the map, in which case None is returned, terminating loop
        //! iteration.
        match self.direction {
            Direction::Down if self.col < self.pipe_map.map[0].len() - 1 => {
                Some((self.row, self.col + 1))
            }
            Direction::Left if self.row < self.pipe_map.map.len() - 1 => {
                Some((self.row + 1, self.col))
            }
            Direction::Right if self.row > 0 => Some((self.row - 1, self.col)),
            Direction::Up if self.col > 0 => Some((self.row, self.col - 1)),
            _ => None,
        }
    }
    fn get_right_orthogonal(&self) -> Option<(usize, usize)> {
        //! Return the right orthogonal point of the current `PipeExplorer`.
        //! It is possible that this will fail because the right orthogonal point is off the map.
        match self.direction {
            Direction::Down if self.col > 0 => Some((self.row, self.col - 1)),
            Direction::Left if self.row > 0 => Some((self.row - 1, self.col)),
            Direction::Right if self.row < self.pipe_map.map.len() - 1 => {
                Some((self.row + 1, self.col))
            }
            Direction::Up if self.col < self.pipe_map.map[0].len() - 1 => {
                Some((self.row, self.col + 1))
            }
            _ => None,
        }
    }

    fn change_direction(&mut self) {
        //! Following the traversal rules of the Day10 problem,
        //! move, change the direction of motion of the `PipeExplorer`.
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
        self.direction = new_direction;
    }
    fn move_in_new_direction(&mut self) {
        //! Move 1-step in the current direction of motion of the Explorer.
        let (new_row, new_col) = self.move_in_direction(&self.direction);
        self.row = new_row;
        self.col = new_col;
        self.steps += 1;
    }

    fn move_next(&mut self) {
        //! Change the direction and move 1-step in that direction in one function.
        //! The reason the methods are split up is because in part2 of the problem, it is necessary
        //! for the methods to be called seperately.
        self.change_direction();
        self.move_in_new_direction();
    }
}

fn find_largest_distance_from_s(mut pipe_1: PipeExplorer, mut pipe_2: PipeExplorer) -> usize {
    //! Given two pipe explorers, which the caller must gurantee are the two pipes connected to S,
    //! will return the largest possible distance from S traveling the loop.
    loop {
        pipe_1.move_next();
        if pipe_1.collided(&pipe_2) {
            break;
        };
        pipe_2.move_next();
        if pipe_1.collided(&pipe_2) {
            break;
        }
    }
    pipe_1.steps.max(pipe_2.steps)
}

fn gather_pipe_locations(
    mut pipe_1: PipeExplorer,
    mut pipe_2: PipeExplorer,
) -> HashSet<(usize, usize)> {
    //! Gather a HashSet containing the ordered pair positions
    //! of all pipes in the actual loop attached to S.
    let mut pipe_locations = HashSet::new();
    pipe_locations.insert(pipe_1.pipe_map.s_position);
    loop {
        pipe_locations.insert((pipe_1.row, pipe_1.col));
        pipe_locations.insert((pipe_2.row, pipe_2.col));
        pipe_1.move_next();
        if pipe_1.collided(&pipe_2) {
            break;
        };
        pipe_2.move_next();
        if pipe_1.collided(&pipe_2) {
            break;
        }
    }
    pipe_locations.insert((pipe_1.row, pipe_1.col));
    pipe_locations.insert((pipe_2.row, pipe_2.col));
    pipe_locations
}

fn walk_loop_interior(
    known_pipe_locations: &HashSet<(usize, usize)>,
    starting_point: (usize, usize),
    interior_points_visited: &mut HashSet<(usize, usize)>,
    map_dimensions: (usize, usize),
) -> Option<()> {
    //! Given a starting point, traverse all points in every
    //! direction, stopping traversal when a known pipe is reached, or an
    //! interior_point already visited.
    let mut stack = vec![starting_point];
    while let Some(current_position) = stack.pop() {
        if interior_points_visited.contains(&current_position)
            || known_pipe_locations.contains(&current_position)
        {
            continue;
        }
        interior_points_visited.insert(current_position);
        //protects against accidentally going off the map.
        let next_steps_are_ok = current_position.0 < map_dimensions.0
            && current_position.0 > 0
            && current_position.1 < map_dimensions.1
            && current_position.1 > 0;
        if !next_steps_are_ok {
            return None;
        }
        let next_positions = [
            (current_position.0 + 1, current_position.1),
            (current_position.0 - 1, current_position.1),
            (current_position.0, current_position.1 + 1),
            (current_position.0, current_position.1 - 1),
        ];
        stack.extend(next_positions);
    }
    Some(())
}

///The loop can be traversed Clockwise or Counterclockwise.
#[derive(Debug, Clone)]
enum LoopDirection {
    Clockwise,
    Counterclockwise,
}

fn compute_surrounded_points(
    mut explorer_guess: PipeExplorer,
    pipe_locations: &HashSet<(usize, usize)>,
    mode: LoopDirection,
) -> Option<()> {
    //! Solves Part2 of the Day10 challenge. The key insight here is that
    //! points that are 'actually in the loop' will always be reachable from the right side
    //! of the loop if traveling clockwise, and from the left side if traveling counterclockwise.
    //!
    //! To solve the problem, traverse the loop either clockwise or counterclockwise.
    //! For each loop point reached, traverse the points to the right or left orthogonal if possible
    //! (stop if another pipe is reached, or if another traversal has already reached the point).
    //! Because it is not trivial to determine which pipe connected to S is the clockwise and counterclockwise pipes,
    //! all 4 possibilities are tried. This means that 2 invocations of this function will return None, and 2 will produce values.
    //! The values for clockwise and counterclockwise traversal should be identical, they are both computed simply for validation.
    let map_dimensions = (
        explorer_guess.pipe_map.map.len(),
        explorer_guess.pipe_map.map[0].len(),
    );
    let (s_position_row, s_position_col) = explorer_guess.pipe_map.s_position;
    let mut found_surrounded_tiles = HashSet::new();
    while !(explorer_guess.row == s_position_row && explorer_guess.col == s_position_col) {
        //The Interior must be walked twice for each explorer position, once
        //before and once after changing the direction of the explorer. Otherwise, a small number
        //of edge cases are missed in the count.
        let orthogonal_point = match mode {
            LoopDirection::Clockwise => explorer_guess.get_right_orthogonal()?,
            LoopDirection::Counterclockwise => explorer_guess.get_left_orthogonal()?,
        };
        walk_loop_interior(
            pipe_locations,
            orthogonal_point,
            &mut found_surrounded_tiles,
            map_dimensions,
        )?;
        //Change the explorers direction and compute the surrounded tiles a second time.
        explorer_guess.change_direction();
        let orthogonal_point = match mode {
            LoopDirection::Clockwise => explorer_guess.get_right_orthogonal()?,
            LoopDirection::Counterclockwise => explorer_guess.get_left_orthogonal()?,
        };
        walk_loop_interior(
            pipe_locations,
            orthogonal_point,
            &mut found_surrounded_tiles,
            map_dimensions,
        )?;
        explorer_guess.move_in_new_direction();
    }
    println!(
        "Surrounded tile count is {} when traveling the loop {:?}",
        found_surrounded_tiles.len(),
        mode
    );
    Some(())
}
