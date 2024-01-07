### Description

I am working through the Advent of Code challenges for 2023, as can be found [here](https://adventofcode.com/2023). So far I have just used Rust, but may do some in Python at some point. I currently have 16 stars. Each day's challenge has two parts. Each day's solution can be found in that day's particular file (for example, day 15 would be found in `day15.rs`).
Running `cargo run` will typically print the solution, provided the input file exists.

Each days solution must implement the trait:

```
trait SolveAdvent {
    ///How to solve part1 of the days puzzle.
    fn solve_part1(path_to_file: &str);
    ///How to solve part2 of the days puzzle.
    fn solve_part2(path_to_file: &str);
}
```

where the two methods each take the file path to the input.
