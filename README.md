### General Description

I am working through the Advent of Code challenges for 2023, as can be found [here](https://adventofcode.com/2023). All solutions are either in Python or Rust. I currently have 18 stars. Each day's challenge has two parts. Each day's solution can be found in that day's particular file (for example, day 15 would be found in `day15.rs`).

#### Rust Solutions

For a day whose solutions is written in Rust, running `cargo run` will typically print the solution, provided the input file exists for Rust solutions, and that day's `SolveAdvent` methods are being invoked in `main.rs`.

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

#### Python Solutions

If a solution is written in Python, then directly pass the file name to the interpreter to run it.
