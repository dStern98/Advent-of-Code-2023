from .base import SolveAdvent
from typing import Optional


def process_input_file_part1(file_content: list[str]) -> list[tuple[str, int]]:
    """
    Process the input file into a list of direction/distance
    to dig.
    """
    parsed_dig_directions = []
    for line in file_content:
        (dig_direction, dig_distance, _) = line.split(" ")
        dig_direction = dig_direction.strip()
        dig_distance = int(dig_distance.strip())
        parsed_dig_directions.append((dig_direction, dig_distance))
    return parsed_dig_directions


def process_input_file_part2(file_content: list[str]) -> list[tuple[str, int]]:
    """
    Process the input file as specified in part2, where the hexadecimal contains
    both the distance and the direction.
    """
    parsed_dig_directions = []
    for line in file_content:
        (_, _, dig_color) = line.split(" ")
        dig_color = dig_color.strip().removeprefix("(#").removesuffix(")")
        dig_direction = DIG_DIRECTION_MAP[dig_color[-1]]
        # Convert the hexadecimal value to an integer.
        dig_distance = int(dig_color[0:len(dig_color) - 1], 16)
        parsed_dig_directions.append((dig_direction, dig_distance))
    return parsed_dig_directions


DIG_DIRECTION_MAP = {
    "0": "R",
    "1": "D",
    "2": "L",
    "3": "U"
}


class Board:
    def __init__(self, dig_directions: list[tuple[str, int]]):
        self.known_dug_positions = self.build_dig_history(dig_directions)
        self.rows_count = max(
            (dig_pair[0] for dig_pair in self.known_dug_positions)) + 1
        self.columns_count = max(
            (dig_pair[1] for dig_pair in self.known_dug_positions)) + 1
        print(f"Board Dimensions: {self.rows_count} x {self.columns_count}")
        self.known_not_dug_positions = set()

    @staticmethod
    def build_dig_history(dig_instructions: list[tuple[str, int]]) -> set[tuple[int, int]]:
        """
        Build a set of all of the position pairs that are dug out initially.
        """
        current_dig_position = (0, 0)
        dig_history: list[tuple[int, int]] = [(0, 0)]
        for (dig_direction, dig_distance) in dig_instructions:
            for _ in range(dig_distance):
                (current_x, current_y) = current_dig_position
                if dig_direction == "R":
                    new_dig_position = (current_x, current_y + 1)
                elif dig_direction == "D":
                    new_dig_position = (current_x + 1, current_y)
                elif dig_direction == "U":
                    new_dig_position = (current_x - 1, current_y)
                else:
                    new_dig_position = (current_x, current_y - 1)
                current_dig_position = new_dig_position
                dig_history.append(new_dig_position)
        # We do not know the dimensions of the dig site ahead of time,
        # so compute the min_x, min_y, and offset all pairs to never be negative.
        min_x = min((pair[0] for pair in dig_history))
        min_y = min((pair[1] for pair in dig_history))
        return {(pair[0] - min_x, pair[1] - min_y) for pair in dig_history}

    def __str__(self) -> str:
        return self.draw_current_location()

    def draw_current_location(
            self,
            row_num: Optional[int] = None,
            col_num: Optional[int] = None) -> str:
        """
        Use the `known_dug_positions` set to draw a representation of the dig
        sites, as shown in the day18 advent example.
        If `row_num`, `col_num` are set, then that spot in the graph will be marked
        with an @.
        """
        board = []
        for i in range(self.rows_count):
            this_row = []
            for j in range(self.columns_count):
                if (i, j) in self.known_dug_positions:
                    this_row.append("#")
                else:
                    this_row.append(".")
            board.append(this_row)
        if row_num is not None and col_num is not None:
            board[row_num][col_num] = "@"
        return "\n".join(["".join(arr) for arr in board])

    def add_not_surrounded_points(self, row_number: int, col_number: int):
        """
        Given a starting position specified by (row_number, col_number), walk the dig site, 
        recording any spots that are not to be dug out to the `known_not_dug_positions` set.
        """
        if (row_number, col_number) in self.known_dug_positions or (row_number, col_number) in self.known_not_dug_positions:
            return
        # To prevent cycles, keep track of where we have traversed.
        spots_already_visited = set()
        spots_to_visit = [(row_number, col_number)]
        while len(spots_to_visit) > 0:
            current_position = spots_to_visit.pop()
            if current_position in spots_already_visited:
                continue
            (current_row_pos, current_col_pos) = current_position
            # Ignore any points that have gone off the dig site in any direction.
            if any((current_row_pos < 0,
                    current_col_pos < 0,
                    current_row_pos >= self.rows_count,
                    current_col_pos >= self.columns_count)):
                continue

            spots_already_visited.add(current_position)
            if (current_row_pos, current_col_pos) not in self.known_dug_positions:
                # If this board position is a '.', then we need to explore
                # all four surrounding squares.
                spots_to_visit.append(
                    (current_row_pos, current_col_pos + 1))
                spots_to_visit.append(
                    (current_row_pos, current_col_pos - 1))
                spots_to_visit.append(
                    (current_row_pos + 1, current_col_pos))
                spots_to_visit.append(
                    (current_row_pos - 1, current_col_pos))

        for spot_visited in spots_already_visited:
            if spot_visited not in self.known_dug_positions:
                self.known_not_dug_positions.add(spot_visited)

    def determine_not_surrounded_points(self):
        """
        Iterate over each of the edges of the dig site (the perimeter), 
        and identify all locations that are not dug out. 
        """
        for col_num in range(self.columns_count):
            # The top side
            self.add_not_surrounded_points(0, col_num)
            # The Bottom side.
            self.add_not_surrounded_points(self.rows_count - 1, col_num)
        for row_num in range(self.rows_count):
            # The left side
            self.add_not_surrounded_points(row_num, 0)
            # the right side
            self.add_not_surrounded_points(row_num, self.columns_count-1)
        # The total dug out number is the total available spots minut the number of spots
        # not dug out.
        print(
            f"Surrounded items equals {self.rows_count * self.columns_count - len(self.known_not_dug_positions)}")


class SolveDay18(SolveAdvent):
    def solve_part1(self):
        parsed_dig_directions = process_input_file_part1(self.file_content)
        dig_board = Board(dig_directions=parsed_dig_directions)
        dig_board.determine_not_surrounded_points()

    def solve_part2(self):
        """
        In theory, the algorithm shown here will work, but it is not performant enough
        to ever complete.
        TODO: Try to get part2 to work.
        """
        parsed_dig_directions = process_input_file_part2(self.file_content)
        dig_board = Board(dig_directions=parsed_dig_directions)
        dig_board.determine_not_surrounded_points()
