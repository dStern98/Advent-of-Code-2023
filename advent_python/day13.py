from .base import SolveAdvent
from dataclasses import dataclass
from typing import Optional, Literal


@dataclass
class PatternScore:
    """
    Represents a known mirror location for a given pattern.
    The `type` specifies whether the mirror is horizontal or vertical.
    The `position` specifies the row/column where the reflection occurs.
    """
    type: Literal["horizontal", "vertical"]
    position: int

    @property
    def points(self):
        if self.type == "horizontal":
            return self.position * 100
        else:
            return self.position


class SolveDay13(SolveAdvent):
    def solve_part1(self):
        total_pattern_score = 0
        for pattern in pattern_generator(self.file_content):
            pattern_parsed = [[char for char in row]
                              for row in pattern.split("\n")]
            if vertical_mirror_score := find_vertical_mirror_score(pattern_parsed):
                total_pattern_score += vertical_mirror_score.points
                continue
            if horizontal_mirror_score := find_horizontal_mirror_score(pattern_parsed):
                total_pattern_score += horizontal_mirror_score.points
                continue
        print(
            f"Total Pattern Score from Mirror Analysis: {total_pattern_score}")

    def solve_part2(self):
        total_pattern_score = 0
        for pattern in pattern_generator(self.file_content):
            pattern_parsed = [[char for char in row]
                              for row in pattern.split("\n")]
            total_pattern_score += find_mirror_by_fixing_smudge(pattern_parsed)
        print(
            f"Total Pattern Score from Mirror Analysis: {total_pattern_score}")


def swap_symbol(current_symbol: str) -> str:
    """
    For Part2 of Day13, swap the symbol. It is the responsiblity of the caller
    to not invoke this function if the current_symbol is a \n, 
    and it will raise a ValueError if one is encountered.
    """
    match current_symbol:
        case ".":
            return "#"
        case "#":
            return "."
        case _:
            raise ValueError(f"Encountered invalid symbol {current_symbol}")


def pattern_generator(file_content: list[str]):
    """
    Generator to allow looping over each pattern
    in a for loop. Concatenate each line in the file
    until a \n is reached, at which point the pattern is yielded.
    """
    pattern = ""
    while len(file_content) > 0:
        next_value = file_content.pop(0)
        if next_value == "\n":
            yield pattern.strip()
            pattern = ""
        else:
            pattern += next_value
    yield pattern.strip()


def find_mirror_by_fixing_smudge(pattern_parsed: list[list[str]]) -> int:
    """
    Find the smudge fix (swapping one symbol) that creates a new reflection
    for the pattern.
    """
    old_horizontal_reflection = find_horizontal_mirror_score(pattern_parsed)
    old_vertical_reflection = find_vertical_mirror_score(pattern_parsed)
    for i in range(len(pattern_parsed)):
        for j in range(len(pattern_parsed[0])):
            # Because the pattern is temporarily mutated, make a deep copy
            # of the list to ensure that the original pattern is not changed.
            pattern_parsed_clone = [[char for char in pattern]
                                    for pattern in pattern_parsed]

            # Ignore \n's (they are not swapped at all)
            if pattern_parsed_clone[i][j] == "\n":
                continue
            # Apply the symbol swap at the i-j location.
            pattern_parsed_clone[i][j] = swap_symbol(
                pattern_parsed_clone[i][j])
            # If a new score is found, return it and end the iteration
            # the compute_new_mirror_score func gurantees that the new_position
            # is different from the old_position.
            if new_horizontal_score := compute_new_mirror_score(
                    old_horizontal_reflection,
                    pattern_parsed_clone,
                    mode="horizontal"):
                return new_horizontal_score
            if new_vertical_score := compute_new_mirror_score(
                    old_vertical_reflection,
                    pattern_parsed_clone,
                    mode="vertical"):
                return new_vertical_score
    # Because the problem prompt gurantees that a new reflection exists for
    # each pattern, if iteration completes without returning, then there is a bug
    # in the code.
    raise RuntimeError("No new mirror detected...")


def compute_new_mirror_score(
        old_reflection: PatternScore | None,
        pattern_parsed_clone: list[list[str]],
        mode: Literal["horizontal", "vertical"]) -> int | None:
    """
    For Part2 of Day13, compute the new mirror score given the "smudge" adjustment.

    It is important to have the original_position that was computed for this pattern, 
    because the instructions ask to only return the new_position if it does not equal
    the original position.
    """
    original_position = None if old_reflection is None else old_reflection.position
    mirror_location_func = find_horizontal_mirror_score if mode == "horizontal" else find_vertical_mirror_score
    # Find the new_reflection (which does not need to exist)
    new_reflection = mirror_location_func(
        pattern_parsed_clone, original_position)
    if new_reflection:
        if not old_reflection:
            # If the old_reflection is None, and new_reflection exists,
            # then we are done.
            return new_reflection.points
        # Otherwise, only return the new_score if the old_position does not equal the new_position.
        if old_reflection.position != new_reflection.position:
            return new_reflection.points


def find_vertical_mirror_score(
        pattern_parsed: list[list[str]],
        original_position: Optional[int] = None
) -> PatternScore | None:
    """
    Finds a vertical reflection, returning the column where the reflection is valid, and the score.
    """
    if vertical_mirror_position := find_mirror_position_in_pattern(pattern_parsed, original_position):
        return PatternScore(
            type="vertical",
            position=vertical_mirror_position)


def find_horizontal_mirror_score(
        pattern_parsed: list[list[str]],
        original_position: Optional[int] = None) -> PatternScore | None:
    """
    Finds a horizontal reflection, returning the row where the reflection is valid
    and the score.
    """
    # Tranpose the pattern for row analysis (the mirror search code only works for columns),
    # but by transposing we can find the correct row answer as well.
    pattern_parsed_transposed = [list(pattern)
                                 for pattern in zip(*pattern_parsed)]
    if horizontal_mirror_position := find_mirror_position_in_pattern(pattern_parsed_transposed, original_position):
        # As the directions explain, the score for a horizontal mirror is 100x the mirror position.
        return PatternScore(
            type="horizontal",
            position=horizontal_mirror_position
        )


def find_mirror_position_in_pattern(
        pattern_parsed: list[list[str]],
        original_position: Optional[int] = None) -> int:
    """
    Given a pattern seperated by new line and by char (hence the list[list[str]]), 
    iterate over the columns and find a valid reflection point. 

    The original_positon field is used for part2, to prevent iteration from terminating
    if the "changed pattern" encounters the same valid reflection point as the original pattern.
    """
    columns_count = len(pattern_parsed[0])
    for col_number in range(1, columns_count):
        # Take the left side of the vertical line, reversing all of the strings
        left_sides = ("".join(pattern[0:col_number][::-1])
                      for pattern in pattern_parsed)
        # Take the right side of the vertical line
        right_sides = ("".join(pattern[col_number:])
                       for pattern in pattern_parsed)

        # A mirror has been found when the right side startswith the left side
        # or vice versa (one side could be longer than the other, and that is fine).
        if all((right_side.startswith(left_side) or left_side.startswith(right_side)
                for (right_side, left_side) in zip(left_sides, right_sides))):
            if not original_position or col_number != original_position:
                return col_number
    return 0
