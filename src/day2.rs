use crate::{read_file_to_string, SolveAdvent};

pub struct Day2;

impl SolveAdvent for Day2 {
    fn solve_part1(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut sum_of_game_powers = 0;
        for line in file_as_str.lines() {
            sum_of_game_powers += possible_game_part1(line);
        }
        println!("Sum of possible game ids: {}", sum_of_game_powers);
    }

    fn solve_part2(path_to_file: &str) {
        let file_as_str = read_file_to_string(path_to_file);
        let mut sum_of_possible_games = 0;
        for line in file_as_str.lines() {
            sum_of_possible_games += min_cube_counts_game_part2(line);
        }
        println!("Sum of possible game ids: {}", sum_of_possible_games);
    }
}

fn possible_game_part1(line: &str) -> isize {
    //! Returns the game_id of the game if it was possible,
    //! otherwise returns 0 (which does not affect the sum).
    let mut line_split_by_colon = line.split(":");
    let game_number = line_split_by_colon
        .next()
        .unwrap()
        .trim()
        .replace("Game ", "")
        .parse::<isize>()
        .unwrap();
    let reveals = line_split_by_colon.next().unwrap().split(";");

    for reveal in reveals {
        let reveal = reveal.trim();
        let mut red_count = 12;
        let mut green_count = 13;
        let mut blue_count = 14;
        let cubes_shown = reveal.trim().split(",").filter(|item| item.len() > 0);
        for cube_shown in cubes_shown {
            let mut cube_shown_split = cube_shown.trim().split(" ");
            let cube_count = cube_shown_split
                .next()
                .unwrap()
                .trim()
                .parse::<isize>()
                .unwrap();
            let cube_color = cube_shown_split.next().unwrap().trim();
            //Given the cube color, subtract the cube count from the numbers of each cube available.
            if cube_color == "blue" {
                blue_count -= cube_count;
            } else if cube_color == "green" {
                green_count -= cube_count;
            } else if cube_color == "red" {
                red_count -= cube_count;
            } else {
                panic!("Recieved unexpected color: {}", cube_color);
            }
        }
        //If any of the counts becomes negative, then the elf showed you more cubes
        //at once then were allowed, so the game is impossible.
        if red_count < 0 || green_count < 0 || blue_count < 0 {
            return 0;
        }
    }
    game_number
}

fn min_cube_counts_game_part2(line: &str) -> isize {
    //! Returns the game_id of the game if it was possible,
    //! otherwise returns 0 (which does not affect the sum).
    let mut line_split_by_colon = line.split(":");
    let _game_number = line_split_by_colon
        .next()
        .unwrap()
        .trim()
        .replace("Game ", "")
        .parse::<isize>()
        .unwrap();
    let reveals = line_split_by_colon.next().unwrap().split(";");
    let mut max_red_count_required = 0;
    let mut max_green_count_required = 0;
    let mut max_blue_count_required = 0;

    for reveal in reveals {
        let reveal = reveal.trim();
        let mut required_red_count = 0;
        let mut required_green_count = 0;
        let mut required_blue_count = 0;
        let cubes_shown = reveal.trim().split(",").filter(|item| item.len() > 0);
        for cube_shown in cubes_shown {
            let mut cube_shown_split = cube_shown.trim().split(" ");
            let cube_count = cube_shown_split
                .next()
                .unwrap()
                .trim()
                .parse::<isize>()
                .unwrap();
            let cube_color = cube_shown_split.next().unwrap().trim();
            //Given the cube color, subtract the cube count from the numbers of each cube available.
            if cube_color == "blue" {
                required_blue_count += cube_count;
            } else if cube_color == "green" {
                required_green_count += cube_count;
            } else if cube_color == "red" {
                required_red_count += cube_count;
            } else {
                panic!("Recieved unexpected color: {}", cube_color);
            }
        }
        //For each reveal round, if any required cube count is larger than the max_required across
        //all the reveals for that cube color, set the max to the new required count.
        if required_red_count > max_red_count_required {
            max_red_count_required = required_red_count;
        }
        if required_blue_count > max_blue_count_required {
            max_blue_count_required = required_blue_count;
        }
        if required_green_count > max_green_count_required {
            max_green_count_required = required_green_count;
        }
    }
    max_red_count_required * max_blue_count_required * max_green_count_required
}
