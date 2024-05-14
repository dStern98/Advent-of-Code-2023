use crate::{read_input_file, SolveAdvent};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

pub struct Day17;

impl SolveAdvent for Day17 {
    fn solve_part1(path_to_file: &str) {
        let file_contents = read_input_file(path_to_file);
        let number_maze = NumberMaze::new(file_contents);
        let now = Instant::now();
        let best_possible_heat_losss = find_best_path_through_maze(number_maze);
        println!("Maze traversal completed in {:?}", now.elapsed());
        println!(
            "Best possible heat loss traversing the maze is: {}",
            best_possible_heat_losss
        );
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
    }
}

#[derive(Debug, Clone)]
struct NumberMaze {
    maze: Vec<Vec<i32>>,
}

impl NumberMaze {
    fn new(file_contents: String) -> NumberMaze {
        let maze = file_contents
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|char| char.to_digit(10).unwrap() as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        NumberMaze { maze }
    }
}

#[derive(Debug, Clone)]
struct MazeProbe<'a> {
    ///History tracks each probes heat loss at each point,
    /// taking into account the last 3 moves the probe took.
    history: HashMap<MapKey, i32>,
    ///Current position x
    row: i32,
    ///Current position y
    col: i32,
    ///The history of this probe
    visited: HashSet<(i32, i32)>,
    ///Total accumulated heat loss along this probes path
    heat_loss: i32,
    ///The last three moves of this probe: Options are L,R,U,D
    last_three_moves: VecDeque<char>,
    ///A reference to the maze
    maze: &'a NumberMaze,
}

fn opposite_direction(dir: &char) -> char {
    match *dir {
        'R' => 'L',
        'L' => 'R',
        'U' => 'D',
        'D' => 'U',
        _ => panic!("Received an illegal direction {dir}"),
    }
}

impl<'a> MazeProbe<'a> {
    fn first_runner(maze: &NumberMaze) -> MazeProbe {
        //! Construct the first maze runner, which is located at
        //! position (0, 0) with a `uuid` of 0, new visited history.
        //! All future Runners through the maze should be spawned from this
        //! returned object, hence why there is no `new` function.
        MazeProbe {
            history: HashMap::new(),
            row: 0,
            col: 0,
            visited: HashSet::from([(0, 0)]),
            heat_loss: -maze.maze[0][0],
            last_three_moves: VecDeque::with_capacity(3),
            maze,
        }
    }

    fn is_inbounds(&self) -> bool {
        let x_in_bounds = self.row >= 0 && self.row < self.maze.maze.len() as i32;
        let y_in_bounds = self.col >= 0 && self.col < self.maze.maze[0].len() as i32;
        x_in_bounds && y_in_bounds
    }

    fn reached_target(&self) -> bool {
        //Return whether the current probe has reached the bottom right of the maze.
        self.row as usize == self.maze.maze.len() - 1
            && self.col as usize == self.maze.maze[0].len() - 1
    }

    fn move_history_repr(&self) -> Option<String> {
        //! Optionally return a string representing the relevant history of this
        //! Probe. This will be the last n items in the history equal to the latest value in the history.
        //! So if the history was `[Up, Down, Down]`, then the returned string would be `DD`.
        if self.last_three_moves.is_empty() {
            return None;
        }
        let mut repr = VecDeque::new();
        let last_move = self.last_three_moves.iter().last().unwrap();
        for char in self.last_three_moves.iter().rev() {
            if char == last_move {
                repr.push_front(*char)
            } else {
                break;
            }
        }

        Some(repr.into_iter().collect::<String>())
    }

    fn visit(&mut self) {
        //! Visit a location in the maze. This involves increasing the runners
        //! running heat loss count, adding the current position to the visited set.
        self.heat_loss += self
            .maze
            .maze
            .get(self.row as usize)
            .unwrap()
            .get(self.col as usize)
            .unwrap();
        self.visited.insert((self.row, self.col));
        //Record the current point in self's history, with the current heat_loss.
        if let Some(move_repr) = self.move_history_repr() {
            let map_key = (self.row, self.col, move_repr);
            self.history.insert(map_key, self.heat_loss);
        }
    }

    fn try_spawn_new(&self, dir_to_move: char) -> Option<Self> {
        //! Attempt to create a new `MazeRunner` in the direction specified by `dir_to_move`.
        //! This may fail for many different reasons, such as that the new runner is off the map,
        //! the move is not allowed by the rules of the problem, or the move would induce a cycle by re-visiting
        //! a location already visited by this probes ancestors.

        //If the last 3 moves were all of the `dir_to_move`, then we cannot move in the `dir_to_move`.
        if self.last_three_moves.len() == 3
            && self
                .last_three_moves
                .iter()
                .all(|last_move| last_move == &dir_to_move)
        {
            return None;
        }

        //Probes are not allowed to reverse direction, which would mean moving in the opposite direction
        //of the last move.
        if !self.last_three_moves.is_empty()
            && self.last_three_moves.iter().last().unwrap() == &opposite_direction(&dir_to_move)
        {
            return None;
        }
        let (new_pos_x, new_pos_y) = match dir_to_move {
            'R' => (self.row, self.col + 1),
            'L' => (self.row, self.col - 1),
            'U' => (self.row - 1, self.col),
            'D' => (self.row + 1, self.col),
            other_direction => unreachable!("Recieved illegal other direction: {other_direction}"),
        };
        let mut new_probe = MazeProbe {
            history: self.history.clone(),
            row: new_pos_x,
            col: new_pos_y,
            visited: self.visited.clone(),
            heat_loss: self.heat_loss,
            last_three_moves: self.last_three_moves.clone(),
            maze: self.maze,
        };
        //Never construct a MazeRunner that is out of bounds, or that is in a cycle
        if !new_probe.is_inbounds() || new_probe.visited.contains(&(new_probe.row, new_probe.col)) {
            return None;
        }

        //Add R to the last three moves
        if new_probe.last_three_moves.len() == 3 {
            new_probe.last_three_moves.pop_front();
        }
        new_probe.last_three_moves.push_back(dir_to_move);
        Some(new_probe)
    }
}

type MapKey = (i32, i32, String);

#[derive(Debug, Clone)]
enum OptimizerOutcomes {
    ///The current probe is the best probe ever found at the current location.
    /// The queue should be cleared of all probes who reached this point
    /// with a worse heat loss (there is guranteed to be at least one probe to remove).
    NewBestProbe(MapKey),
    ///The optimizer has determined that the current probe being examined
    /// is worse than a previous probe. The current probe should be killed.
    BadCurrentProbe,
    ///The Optimizer cannot determine any optimizations to perform
    InsufficientInfo,
}

fn optimize_using_history(
    maze_runner: &MazeProbe,
    optimizer: &mut HashMap<MapKey, i32>,
) -> OptimizerOutcomes {
    let move_repr = match maze_runner.move_history_repr() {
        Some(repr) => repr,
        None => return OptimizerOutcomes::InsufficientInfo,
    };
    let map_key = (maze_runner.row, maze_runner.col, move_repr.clone());
    if let Some(best_observed_loss) = optimizer.get_mut(&map_key) {
        if *best_observed_loss > maze_runner.heat_loss {
            //If the current min is greater than heat_loss of current probe, then change
            // the recorded min to the heat loss.
            *best_observed_loss = maze_runner.heat_loss;
            //Return Some to indicate that a new best heat_loss has been recorded for the current map_key.
            //This means that some probes can be removed from the queue.
            return OptimizerOutcomes::NewBestProbe(map_key);
        }
        //If current_min is <= the maze_runners heat loss, than the current maze runner
        //should be removed from the queue.
        return OptimizerOutcomes::BadCurrentProbe;
    }

    if move_repr.len() > 1 {
        //It is worth remembering that a Probe whose last 3 moves was UUD will have all of the same
        //freedoms as a DDD and more! Therefore, if the current probe has either moved 2 or 3 moves in the same
        //direction in a row, we can still check if a probe that has moved only 1 consecutive moves in the same direction
        //did better than this probe. If so, we can still kill this probe.
        let last_move = move_repr.chars().last().unwrap().to_string();
        let mut extra_move_reprs = vec![last_move.clone()];
        if move_repr.len() == 3 {
            extra_move_reprs.push(format!("{}{}", last_move, last_move));
        }
        for extra_move_repr in extra_move_reprs {
            let map_key2 = (maze_runner.row, maze_runner.col, extra_move_repr);
            if let Some(best_observed_loss) = optimizer.get_mut(&map_key2) {
                if *best_observed_loss <= maze_runner.heat_loss {
                    //If current_min is <= the maze_runners heat loss, than the current maze runner
                    //should be removed from the queue.
                    return OptimizerOutcomes::BadCurrentProbe;
                }
            }
        }
    }

    //If we have not already returned, then the current map_key is not in the optimizer at all,
    //so just insert a new entry for it and return.
    optimizer.insert(map_key, maze_runner.heat_loss);
    OptimizerOutcomes::InsufficientInfo
}

fn find_best_path_through_maze(number_maze: NumberMaze) -> i32 {
    //! Perform a Depth-first search. Use the optimizer (a hashmap mapping (row, col, repr) to best observed heat loss).
    //! The mentioned repr is a string representing how many consecutive moves in a row the probe has moved.
    //! Without the optimizer and the associated `optimize_using_history` function, this function could take 1000 years
    //! to complete.
    let mut min_accumulated_heat_loss = i32::MAX;
    let mut maze_probe_bfs_queue = VecDeque::from([MazeProbe::first_runner(&number_maze)]);
    let mut optimizer = HashMap::new();
    while let Some(mut maze_runner) = maze_probe_bfs_queue.pop_front() {
        maze_runner.visit();
        if maze_runner.reached_target() {
            min_accumulated_heat_loss = min_accumulated_heat_loss.min(maze_runner.heat_loss);
            continue;
        }
        match optimize_using_history(&maze_runner, &mut optimizer) {
            OptimizerOutcomes::BadCurrentProbe => {
                //If the current probe is bad, then do not spawn new probes
                //from the current probe.
                continue;
            }
            OptimizerOutcomes::NewBestProbe(map_key) => {
                maze_probe_bfs_queue.retain(|probe| {
                    //Remove any probes from the probes queue if the probe at the current map_key had a recorded heat loss
                    //greater than the current maze_runner.
                    if let Some(recorded_heat_loss) = probe.history.get(&map_key) {
                        if *recorded_heat_loss > maze_runner.heat_loss {
                            return false;
                        }
                    }
                    true
                });
            }
            OptimizerOutcomes::InsufficientInfo => {}
        }

        let next_moves = [
            maze_runner.try_spawn_new('R'),
            maze_runner.try_spawn_new('L'),
            maze_runner.try_spawn_new('U'),
            maze_runner.try_spawn_new('D'),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<MazeProbe>>();
        maze_probe_bfs_queue.extend(next_moves);
    }
    min_accumulated_heat_loss
}
