use crate::{read_input_file, SolveAdvent};
use std::collections::{HashMap, HashSet};

pub struct Day22;

impl SolveAdvent for Day22 {
    fn solve_part1(path_to_file: &str) {
        let file_contents = read_input_file(path_to_file);
        let mut bricks = file_contents
            .lines()
            .zip((0..).into_iter().cycle())
            .map(|(line, uuid)| Brick::from_line(line, uuid))
            .collect::<Vec<_>>();
        bricks.sort_by_key(|brick| -brick.lower_bound.2);
        let bricks = descend_bricks(bricks);
        let bricks_safe_to_remove = find_bricks_safe_to_disintegrate(bricks);
        println!("There are {} bricks safe to remove", bricks_safe_to_remove);
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
    }
}

///Represents a single brick in the pile
#[derive(Debug, Copy, Clone)]
struct Brick {
    ///a unique number useful for debugging purposes
    uuid: i32,
    lower_bound: (i32, i32, i32),
    upper_bound: (i32, i32, i32),
}

impl Brick {
    fn find_supported_bricks(&self, bricks_directly_above: &[&Brick]) -> HashSet<i32> {
        //! Return a set of all the bricks above `self` that have an xy-overlap.
        //! This means that `self` is one of the bricks that holds up all of the bricks in `bricks_self_holds_up`
        let mut bricks_self_holds_up = HashSet::new();
        for brick_directly_above in bricks_directly_above {
            if brick_directly_above.check_xy_overlap(self) {
                bricks_self_holds_up.insert(brick_directly_above.uuid);
            }
        }
        bricks_self_holds_up
    }
    fn move_brick_z_position(&mut self, new_z_lower_bound: i32) {
        //! Mutate the current brick's z-position so that the lowest end of the z
        //! is at the `new_z_lower_bound` position. The height of the brick
        //! is preserved.
        let z_delta = self.upper_bound.2 - self.lower_bound.2;
        self.lower_bound.2 = new_z_lower_bound;
        self.upper_bound.2 = new_z_lower_bound + z_delta;
    }

    fn x_overlap(&self, other: &Brick) -> bool {
        //! Check if two bricks overlap on the x-axis.
        let x_overlap1 =
            self.lower_bound.0 <= other.lower_bound.0 && self.upper_bound.0 >= other.lower_bound.0;
        let x_overlap2 =
            other.lower_bound.0 <= self.lower_bound.0 && other.upper_bound.0 >= self.lower_bound.0;
        x_overlap1 || x_overlap2
    }

    fn y_overlap(&self, other: &Brick) -> bool {
        //! Check if two bricks overlap on the y-axis.
        let y_overlap1 =
            self.lower_bound.1 <= other.lower_bound.1 && self.upper_bound.1 >= other.lower_bound.1;
        let y_overlap2 =
            other.lower_bound.1 <= self.lower_bound.1 && other.upper_bound.1 >= self.lower_bound.1;

        y_overlap1 || y_overlap2
    }

    fn z_overlap(&self, other: &Brick) -> bool {
        //! Check if two bricks overlap on the z-axis.
        let z_overlap1 =
            self.lower_bound.2 <= other.lower_bound.2 && self.upper_bound.2 >= other.lower_bound.2;
        let z_overlap2 =
            other.lower_bound.2 <= self.lower_bound.2 && other.upper_bound.2 >= self.lower_bound.2;
        z_overlap1 || z_overlap2
    }

    fn check_xy_overlap(&self, other: &Brick) -> bool {
        self.x_overlap(other) && self.y_overlap(other)
    }

    fn check_for_collision(&self, other: &Brick) -> bool {
        self.x_overlap(other) && self.y_overlap(other) && self.z_overlap(other)
    }
    fn from_line(line: &str, uuid: i32) -> Brick {
        //! Convert a single line of the input file into a `Brick`.
        let mut split_by_tilda = line.split('~');
        let [x_lower, y_lower, z_lower]: [i32; 3] = split_by_tilda
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let [x_upper, y_upper, z_upper]: [i32; 3] = split_by_tilda
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Brick {
            uuid,
            lower_bound: (x_lower, y_lower, z_lower),
            upper_bound: (x_upper, y_upper, z_upper),
        }
    }
}

fn lower_brick_as_far_as_possible(
    final_brick_positions: &mut Vec<Brick>,
    mut brick_to_descend: Brick,
) {
    //! Given a brick to descend, descend the brick until it collides with another brick already
    //! settled, at which point we stop the descending. If another brick is never hit, the loop
    //! still will terminate when it runs out of settled bricks to compare to.

    //If the final_brick_positions vec is not empty, then we need to descend the current
    //brick until it collides with a brick.
    //Make sure that the final_brick_positions vector is sorted by the z-axis upper bound.
    final_brick_positions.sort_by_key(|brick| brick.upper_bound.2);
    let mut settled_brick_cursor = final_brick_positions.len() - 1;
    while let Some(brick_to_compare_to) = final_brick_positions.get(settled_brick_cursor) {
        brick_to_descend.move_brick_z_position(brick_to_compare_to.upper_bound.2);
        if brick_to_descend.check_for_collision(brick_to_compare_to) {
            //If they collide, then the correct position of the lowest_remaining_brick is 1 above the current_settled_brick.
            brick_to_descend.move_brick_z_position(brick_to_compare_to.upper_bound.2 + 1);
            //if the settled_brick_cursor is at the end of the vec, then push onto the vec
            if settled_brick_cursor == final_brick_positions.len() - 1 {
                final_brick_positions.push(brick_to_descend);
            } else {
                //Otherwise just insert at the correct position
                final_brick_positions.insert(settled_brick_cursor + 1, brick_to_descend);
            }
            break;
        } else {
            //If the current brick does not collide, then we continue with the next from
            //last brick in the lowest_remaining_brick array
            if settled_brick_cursor == 0 {
                brick_to_descend.move_brick_z_position(1);
                final_brick_positions.insert(0, brick_to_descend);
                break;
            }
            settled_brick_cursor -= 1;
        }
    }
}

fn descend_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    //! Descend all bricks until no more bricks can be moved downward.
    //! The lowest z-position a brick can possibly have is `z=1`, as
    //! `z=0` is the ground.
    let mut final_brick_positions = Vec::new();
    while let Some(mut brick_to_descend) = bricks.pop() {
        if final_brick_positions.is_empty() {
            //If the final_brick_positions vec is empty, then we are operating on the lowest brick,
            //which we know will end up at the z_position of 1.
            brick_to_descend.move_brick_z_position(1);
            final_brick_positions.push(brick_to_descend)
        } else {
            lower_brick_as_far_as_possible(&mut final_brick_positions, brick_to_descend);
        }
    }
    final_brick_positions
}

fn find_bricks_safe_to_disintegrate(bricks: Vec<Brick>) -> i32 {
    //! Return the number of bricks that can safely be disintegrated without another
    //! brick falling as a result.
    let mut brick_height_map: HashMap<i32, Vec<&Brick>> = HashMap::new();
    for brick in bricks.iter() {
        brick_height_map
            .entry(brick.lower_bound.2)
            .or_default()
            .push(brick);
    }
    //The brick dependency map stores the uuids of every brick that brick with uuid key
    //holds up.
    let mut brick_dependency_map = HashMap::new();
    let mut bricks_safe_to_remove = 0;
    for brick_to_assess in bricks.iter() {
        if let Some(bricks_directly_above) =
            brick_height_map.get(&(brick_to_assess.upper_bound.2 + 1))
        {
            let supported_bricks = brick_to_assess.find_supported_bricks(&bricks_directly_above);
            if !supported_bricks.is_empty() {
                brick_dependency_map.insert(
                    brick_to_assess.uuid,
                    brick_to_assess.find_supported_bricks(&bricks_directly_above),
                );
                continue;
            }
        }
        //If there are 0 bricks directly above the brick_to_assess, then we know that the brick_to_assess
        // could safely be removed.
        bricks_safe_to_remove += 1;
    }
    for (brick_uuid, bricks_held_up) in brick_dependency_map.iter() {
        if !bricks_held_up.is_empty() {
            //Build a hashset of all the help up bricks, excluding the brick being examined.
            let all_other_holders: HashSet<i32> = brick_dependency_map
                .iter()
                .filter_map(|(k, v)| {
                    if k != brick_uuid {
                        return Some(v.clone());
                    }
                    None
                })
                .flatten()
                .collect::<HashSet<_>>();
            //If the bricks_held_up by the current brick being examined is a subset of the bricks held
            //up by others, then removing the current brick would NOT cause any bricks to fall (making it safe to remove)
            if bricks_held_up.is_subset(&all_other_holders) {
                bricks_safe_to_remove += 1;
            }
        }
    }
    bricks_safe_to_remove
}
