use crate::{read_file_to_string, SolveAdvent};
use std::collections::HashSet;
use std::fmt;

pub struct Day16;

impl SolveAdvent for Day16 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let input_map = parse_input_into_map(&file_as_str);
        let starting_beam = LightBeam {
            row: 0,
            column: 0,
            direction: Direction::Right,
            map: &input_map,
        };

        let energized_tiles_count = find_energized_tiles(starting_beam);
        println!(
            "There are {} energized tiles for Part1",
            energized_tiles_count
        );
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let input_map = parse_input_into_map(&file_as_str);
        let mut max_energized_tiles_count = 0;
        //For Part2, we iterator over all possible starting configuration to find
        //the configuration that generates the most energized tiles.
        let starting_beam_configurations = generate_unique_starting_configurations(&input_map);
        for (starting_direction, row_col_iterator) in starting_beam_configurations {
            for (row, column) in row_col_iterator {
                let light_beam = LightBeam {
                    row,
                    column,
                    map: &input_map,
                    direction: starting_direction.clone(),
                };
                let energized_tiles = find_energized_tiles(light_beam);
                if energized_tiles > max_energized_tiles_count {
                    max_energized_tiles_count = energized_tiles;
                }
            }
        }
        println!(
            "Maximum energized tiles in any starting configuration is {} for Part2",
            max_energized_tiles_count
        );
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn parse_input_into_map(file_as_str: &str) -> Vec<Vec<char>> {
    file_as_str
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

impl Direction {
    fn repr(&self) -> &'static str {
        //! For hash/eq purposes, it is easier to just return
        //! a string representation of the direction.
        match self {
            Direction::Down => "Down",
            Direction::Up => "Up",
            Direction::Right => "Right",
            Direction::Left => "Left",
        }
    }
    fn forward_slash_reflection(&self) -> Self {
        //! Rotate the beam direction using the rules of a reflection
        //! against a forward slash.
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn backwards_slash_rotation(&self) -> Self {
        //!Rotate the beam direction using the rules of a reflection
        //! against a backwards slash.
        match self {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

///Represents a Single Light Beam
#[derive(Clone, Debug)]
struct LightBeam<'a> {
    row: usize,
    column: usize,
    ///The direction the light beam is traveling
    direction: Direction,
    map: &'a Vec<Vec<char>>,
}

impl<'a> fmt::Display for LightBeam<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Row: {}, Column: {} , Direction: {:?}",
            self.row, self.column, self.direction
        )
    }
}

impl<'a> LightBeam<'a> {
    fn get_mirror_at_position(&self) -> Option<&char> {
        //! Returns the corresponding character from the problem input.
        if let Some(row) = self.map.get(self.row) {
            if let Some(mirror) = row.get(self.column) {
                return Some(mirror);
            }
        }
        None
    }
    fn try_build_next(&self, direction: Direction) -> Option<Self> {
        //! Attempt to build the next `LightBeam`.
        match direction {
            Direction::Down => {
                if self.row >= self.map.len() - 1 {
                    return None;
                }
                return Some(LightBeam {
                    row: self.row + 1,
                    column: self.column,
                    map: self.map,
                    direction,
                });
            }
            Direction::Left => {
                if self.column == 0 {
                    return None;
                }
                return Some(LightBeam {
                    row: self.row,
                    column: self.column - 1,
                    map: self.map,
                    direction,
                });
            }
            Direction::Right => {
                if self.column >= self.map[0].len() - 1 {
                    return None;
                }
                return Some(LightBeam {
                    row: self.row,
                    column: self.column + 1,
                    map: self.map,
                    direction,
                });
            }
            Direction::Up => {
                if self.row == 0 {
                    return None;
                }
                return Some(LightBeam {
                    row: self.row - 1,
                    column: self.column,
                    map: self.map,
                    direction,
                });
            }
        }
    }

    fn react_to_mirror(self) -> impl IntoIterator<Item = Self> {
        //! Consumes the current light beam, and returns an IntoIterator
        //! type which gives the next `LightBeam`s. A Iterator/Collection is required
        //! because in the case of the beam enountering a splitter, two beams could be created.
        let current_symbol = self
            .get_mirror_at_position()
            .expect("Invalid LightBeam was constructed");
        let next_directions = match current_symbol {
            &'.' => {
                //Do nothing
                vec![self.direction.clone()]
            }
            &'/' => {
                //Reflect
                vec![self.direction.forward_slash_reflection()]
            }
            &'\\' => {
                //Reflect
                vec![self.direction.backwards_slash_rotation()]
            }
            &'-' => match self.direction {
                //Split the beam
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],

                _ => vec![self.direction.clone()],
            },
            &'|' => match self.direction {
                //Split the beam.
                Direction::Right | Direction::Left => vec![Direction::Up, Direction::Down],
                _ => vec![self.direction.clone()],
            },
            other => panic!("Encountered unexpected symbol {}", other),
        };
        let next_light_beams = next_directions
            .into_iter()
            .filter_map(|direction| self.try_build_next(direction))
            .collect::<Vec<_>>();
        next_light_beams
    }
}

fn find_energized_tiles(starting_beam: LightBeam) -> usize {
    //! Given a starting beam, returns the number of energized tiles
    //! for the given starting beam.
    //Store a HashSet of row, column, direction to prevent infinite cycles, which are possible
    let mut energized_sites = HashSet::new();
    energized_sites.insert((
        starting_beam.row,
        starting_beam.column,
        Direction::Right.repr(),
    ));
    let mut lasers = vec![starting_beam];
    while !lasers.is_empty() {
        lasers = lasers
            .into_iter()
            .flat_map(|laser| laser.react_to_mirror())
            .filter(|laser_beam| {
                if !energized_sites.contains(&(
                    laser_beam.row,
                    laser_beam.column,
                    laser_beam.direction.repr(),
                )) {
                    energized_sites.insert((
                        laser_beam.row,
                        laser_beam.column,
                        laser_beam.direction.repr(),
                    ));
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
    }
    //The problem actually wants the number of unique locations reached, regardless
    //of the direction of the beam at the time.
    let unique_visited_locations = energized_sites
        .into_iter()
        .map(|(row, column, _)| (row, column))
        .collect::<HashSet<_>>();

    unique_visited_locations.len()
}

fn generate_unique_starting_configurations(
    input_map: &Vec<Vec<char>>,
) -> [(Direction, Vec<(usize, usize)>); 4] {
    //! Part2 says that the beam can start moving down from the top row, moving up from the bottom,
    //! row, moving right from the far left column, and moving left from the far right column.
    //! Build out all of these possibilities.
    let (map_rows, map_columns) = (input_map.len(), input_map[0].len());
    [
        (
            Direction::Right,
            (0..map_rows)
                .map(|row| (row, 0))
                .into_iter()
                .collect::<Vec<_>>(),
        ),
        (
            Direction::Left,
            (0..map_rows)
                .map(|row| (row, map_columns - 1))
                .into_iter()
                .collect::<Vec<_>>(),
        ),
        (
            Direction::Up,
            (0..map_columns)
                .map(|column| (map_rows - 1, column))
                .into_iter()
                .collect::<Vec<_>>(),
        ),
        (
            Direction::Down,
            (0..map_columns)
                .map(|column| (0, column))
                .into_iter()
                .collect::<Vec<_>>(),
        ),
    ]
}
