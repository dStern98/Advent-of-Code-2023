use crate::{read_input_file, SolveAdvent};
use std::collections::HashSet;

pub struct Day21;

///Stores the actual input map for the problem
#[derive(Debug, Clone)]
struct Garden {
    map: Vec<Vec<char>>,
    row_count: usize,
    column_count: usize,
}

///An explorer traverses the garden, mapping
///potentially accessable garden plots.
#[derive(Debug, Clone)]
struct GardenExplorer<'a> {
    row: usize,
    column: usize,
    garden: &'a Garden,
}

#[derive(Debug, Clone)]
struct GardenExplorerInfinite<'a> {
    row: isize,
    column: isize,
    garden: &'a Garden,
}

impl<'a> GardenExplorerInfinite<'a> {
    fn is_valid_garden_plot(&self) -> bool {
        //! If the current `GardenExplorer` is in a valid garden plot,
        //! then returns True. This can be false either if the row, or column is off the garden map,
        //! or if the current position is a rock, symbolized by a `#`
        let row_count = self.garden.row_count as isize;
        let column_count = self.garden.column_count as isize;
        let mut rows_adjusted = self.row % row_count;
        let mut column_adjusted = self.column % column_count;
        if rows_adjusted < 0 {
            rows_adjusted += row_count;
        }
        if column_adjusted < 0 {
            column_adjusted += column_count;
        }

        if let Some(position) = self
            .garden
            .get_position(rows_adjusted as usize, column_adjusted as usize)
        {
            if position == &'#' {
                return false;
            }
            return true;
        }
        false
    }

    fn left(&self) -> Option<Self> {
        //! Attempts to go left one from the current garden explorer.
        //! This can fail if the left one explorer is invalid for some reason (off the map,
        //! or on a rock rather than a garden plot.)
        let left_one = GardenExplorerInfinite {
            row: self.row,
            column: self.column - 1,
            garden: self.garden,
        };
        if left_one.is_valid_garden_plot() {
            return Some(left_one);
        }
        None
    }

    fn right(&self) -> Option<Self> {
        //! The same as `left_one` but trying to go right.
        let right_one = GardenExplorerInfinite {
            row: self.row,
            column: self.column + 1,
            garden: self.garden,
        };
        if right_one.is_valid_garden_plot() {
            return Some(right_one);
        }
        None
    }

    fn up(&self) -> Option<Self> {
        //! Try to go up from the current `GardenExplorer`.
        let up_one = GardenExplorerInfinite {
            row: self.row - 1,
            column: self.column,
            garden: self.garden,
        };
        if up_one.is_valid_garden_plot() {
            return Some(up_one);
        }
        None
    }

    fn down(&self) -> Option<Self> {
        //! Try to go down from the current `GardenExplorer`.
        let down_one = GardenExplorerInfinite {
            row: self.row + 1,
            column: self.column,
            garden: self.garden,
        };
        if down_one.is_valid_garden_plot() {
            return Some(down_one);
        }
        None
    }

    fn next_steps(&self) -> impl Iterator<Item = Self> {
        //! Try to move in all 4 possible directions at once, returning
        //! all of the valid moves.
        [self.up(), self.down(), self.left(), self.right()]
            .into_iter()
            .flatten()
    }
}

impl<'a> GardenExplorer<'a> {
    fn is_valid_garden_plot(&self) -> bool {
        //! If the current `GardenExplorer` is in a valid garden plot,
        //! then returns True. This can be false either if the row, or column is off the garden map,
        //! or if the current position is a rock, symbolized by a `#`
        if let Some(position) = self.garden.get_position(self.row, self.column) {
            if position == &'#' {
                return false;
            }
            return true;
        }
        false
    }

    fn left(&self) -> Option<Self> {
        //! Attempts to go left one from the current garden explorer.
        //! This can fail if the left one explorer is invalid for some reason (off the map,
        //! or on a rock rather than a garden plot.)
        if self.column == 0 {
            return None;
        }
        let left_one = GardenExplorer {
            row: self.row,
            column: self.column - 1,
            garden: self.garden,
        };
        if left_one.is_valid_garden_plot() {
            return Some(left_one);
        }
        None
    }

    fn right(&self) -> Option<Self> {
        //! The same as `left_one` but trying to go right.
        let right_one = GardenExplorer {
            row: self.row,
            column: self.column + 1,
            garden: self.garden,
        };
        if right_one.is_valid_garden_plot() {
            return Some(right_one);
        }
        None
    }

    fn up(&self) -> Option<Self> {
        //! Try to go up from the current `GardenExplorer`.
        if self.row == 0 {
            return None;
        }
        let up_one = GardenExplorer {
            row: self.row - 1,
            column: self.column,
            garden: self.garden,
        };
        if up_one.is_valid_garden_plot() {
            return Some(up_one);
        }
        None
    }

    fn down(&self) -> Option<Self> {
        //! Try to go down from the current `GardenExplorer`.
        let down_one = GardenExplorer {
            row: self.row + 1,
            column: self.column,
            garden: self.garden,
        };
        if down_one.is_valid_garden_plot() {
            return Some(down_one);
        }
        None
    }

    fn next_steps(&self) -> impl Iterator<Item = Self> {
        //! Try to move in all 4 possible directions at once, returning
        //! all of the valid moves.
        [self.up(), self.down(), self.left(), self.right()]
            .into_iter()
            .flatten()
    }
}

impl Garden {
    fn new(file_as_str: &str) -> Garden {
        let map = file_as_str
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (row_count, column_count) = (map.len(), map[0].len());
        Garden {
            map,
            row_count,
            column_count,
        }
    }

    fn s_position(&self) -> (usize, usize) {
        for (row_number, row) in self.map.iter().enumerate() {
            for (column_number, garden_item) in row.iter().enumerate() {
                if garden_item == &'S' {
                    return (row_number, column_number);
                }
            }
        }
        panic!("Garden Map did not contain an S");
    }

    fn get_position(&self, row: usize, col: usize) -> Option<&char> {
        if let Some(row) = self.map.get(row) {
            if let Some(col_item) = row.get(col) {
                return Some(col_item);
            }
        }
        None
    }
}

impl SolveAdvent for Day21 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_input_file(path_to_file);
        let garden = Garden::new(&file_as_str);
        let position_of_s = garden.s_position();
        find_accessable_garden_plots(64, garden, position_of_s);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_input_file(path_to_file);
        let garden = Garden::new(&file_as_str);
        let position_of_s = garden.s_position();
        find_accessable_garden_plots_infinite(1000, garden, position_of_s);
    }
}

fn find_accessable_garden_plots(steps: usize, garden: Garden, position_of_s: (usize, usize)) {
    //! Start with a `GardenExplorer` at the position of S, which is the start.
    //!For each step iteration, try to move each `GardenExplorer` in all 4 directions.
    //! Use a HashSet to remove all collisions, which prevents the exponential growth of
    //! the number of `GardenExplorer`s.
    let mut step_tracker = vec![GardenExplorer {
        row: position_of_s.0,
        column: position_of_s.1,
        garden: &garden,
    }];
    let mut unique_garden_plot_tracker = HashSet::new();
    unique_garden_plot_tracker.insert((step_tracker[0].row, step_tracker[0].column));

    for _ in 0..steps {
        unique_garden_plot_tracker.clear();
        step_tracker = step_tracker
            .into_iter()
            .flat_map(|garden_explorer| garden_explorer.next_steps())
            .filter(|garden_explorer| {
                if !unique_garden_plot_tracker
                    .contains(&(garden_explorer.row, garden_explorer.column))
                {
                    unique_garden_plot_tracker
                        .insert((garden_explorer.row, garden_explorer.column));
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
    }
    println!(
        "After {} steps, there are {} uniquely accessable garden plots",
        steps,
        unique_garden_plot_tracker.len()
    );
}

fn find_accessable_garden_plots_infinite(
    steps: usize,
    garden: Garden,
    position_of_s: (usize, usize),
) {
    //! Start with a `GardenExplorer` at the position of S, which is the start.
    //!For each step iteration, try to move each `GardenExplorer` in all 4 directions.
    //! Use a HashSet to remove all collisions, which prevents the exponential growth of
    //! the number of `GardenExplorer`s.
    //! Sadly this code works correctly, but is not efficient enough to complete in time.
    let mut step_tracker = vec![GardenExplorerInfinite {
        row: position_of_s.0 as isize,
        column: position_of_s.1 as isize,
        garden: &garden,
    }];
    let mut unique_garden_plot_tracker = HashSet::with_capacity(2_000_000);
    unique_garden_plot_tracker.insert((step_tracker[0].row, step_tracker[0].column));

    for current_steps in 0..steps {
        unique_garden_plot_tracker.clear();
        step_tracker = step_tracker
            .into_iter()
            .flat_map(|garden_explorer| garden_explorer.next_steps())
            .filter(|garden_explorer| {
                if !unique_garden_plot_tracker
                    .contains(&(garden_explorer.row, garden_explorer.column))
                {
                    unique_garden_plot_tracker
                        .insert((garden_explorer.row, garden_explorer.column));
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
        println!(
            "After {} steps, there are {} accessable garden plots",
            current_steps,
            step_tracker.len()
        );
    }
    println!(
        "After {} steps, there are {} uniquely accessable garden plots",
        steps,
        unique_garden_plot_tracker.len()
    );
}
