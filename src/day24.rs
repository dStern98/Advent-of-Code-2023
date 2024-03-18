use crate::{read_file_to_string, SolveAdvent};

pub struct Day24;

#[derive(Debug, Clone)]
struct Hailstone {
    x: f64,
    y: f64,
    _z: f64,
    vx: f64,
    vy: f64,
    _vz: f64,
}

impl Hailstone {
    #[allow(dead_code)]
    fn move_t(&mut self, t: f64) {
        self.x += t * self.vx;
        self.y += t * self.vy;
        self._z += t * self._vz;
    }
    fn future_x_y_collision(&self, other: &Self) -> Option<(f64, f64)> {
        //! Check if the two hailstones collide, excludingt the Z-axis, in the
        //! future for the two hailstones. If they do collide, returns the `(x,y)`
        //! pair of the collision.
        //!
        //! If equation 1 is: `y = ax + b`
        //! and equation 2 is `y = cx + d`,
        //! then it can be shown that `x_col = (d-b)/ (a-c)`
        //! where `a != c`

        //Assign a,c,d,c to make the logic easier to reason about
        let a = self.vy / self.vx;
        let b = self.y - (self.vy * self.x / self.vx);
        let c = other.vy / other.vx;
        let d = other.y - (other.vy * other.x / other.vx);
        //Corresponds to parallel lines, which never collide
        if a == c {
            return None;
        }

        let x_collision = (d - b) / (a - c);

        //Check that x_collision is in the future for both particles.
        let t_x_collision_self = (x_collision - self.x) / self.vx;
        let t_x_collision_other = (x_collision - other.x) / other.vx;
        if t_x_collision_self < 0.0 || t_x_collision_other < 0.0 {
            return None;
        }

        let y_collision = a * x_collision + b;
        Some((x_collision, y_collision))
    }

    fn from_line(line: &str) -> Hailstone {
        //! Construct a Hailstone by processing a line of the input file.
        let mut position_velocity_split = line.split('@');
        let positions = position_velocity_split
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|num| num.trim().parse::<i64>().unwrap() as f64)
            .collect::<Vec<_>>();
        let velocities = position_velocity_split
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|num| num.trim().parse::<i64>().unwrap() as f64)
            .collect::<Vec<_>>();
        let [vx, vy, vz]: [f64; 3] = velocities.try_into().unwrap();
        let [x, y, z]: [f64; 3] = positions.try_into().unwrap();
        Hailstone {
            x,
            y,
            _z: z,
            vx,
            vy,
            _vz: vz,
        }
    }
}

impl SolveAdvent for Day24 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let hailstones = file_as_str
            .lines()
            .map(Hailstone::from_line)
            .collect::<Vec<_>>();
        let collision_box = (200000000000000.0, 400000000000000.0);
        count_collisions_part_1(&hailstones, collision_box, collision_box);
    }

    fn solve_part2(path_to_file: &str) {
        let _ = path_to_file;
    }
}

fn count_collisions_part_1(
    hailstones: &[Hailstone],
    (x_col_lower, x_col_upper): (f64, f64),
    (y_col_lower, y_col_upper): (f64, f64),
) {
    //! Count all hailstone collisions that occur within the required
    //! target area.
    let mut future_hailstone_collisions = 0;
    for (hailstone_num, hailstone) in hailstones.iter().enumerate() {
        for (hailstone_num2, hailstone2) in hailstones.iter().enumerate() {
            if hailstone_num < hailstone_num2 {
                if let Some((col_x, col_y)) = hailstone.future_x_y_collision(hailstone2) {
                    if x_col_lower <= col_x
                        && col_x <= x_col_upper
                        && y_col_lower <= col_y
                        && col_y <= y_col_upper
                    {
                        future_hailstone_collisions += 1;
                    }
                }
            }
        }
    }
    println!(
        "Total hailstone crossings part1 is {}",
        future_hailstone_collisions
    );
}
