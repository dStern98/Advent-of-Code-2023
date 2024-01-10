from .base import SolveAdvent


def process_input_file_part1(file_content: list[str]) -> list[tuple[str, int]]:
    parsed_dig_directions = []
    for line in file_content:
        (dig_direction, dig_distance, _) = line.split(" ")
        dig_direction = dig_direction.strip()
        dig_distance = int(dig_distance.strip())
        parsed_dig_directions.append((dig_direction, dig_distance))

    return parsed_dig_directions


def process_input_file_part2(file_content: list[str]) -> list[tuple[str, int]]:
    parsed_dig_directions = []
    for line in file_content:
        (_, _, dig_color) = line.split(" ")
        dig_color = dig_color.strip().removeprefix("(#").removesuffix(")")
        dig_direction = DIG_DIRECTION_MAP[dig_color[-1]]
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
        self.board = self.build_dig_history(dig_directions)
        self.rows_count = max((dig_pair[0] for dig_pair in self.board)) + 1
        self.columns_count = max((dig_pair[1] for dig_pair in self.board)) + 1

    @staticmethod
    def build_dig_history(dig_instructions: list[tuple[str, int]]) -> set[tuple[int, int]]:
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
        min_x = min((pair[0] for pair in dig_history))
        min_y = min((pair[1] for pair in dig_history))
        return {(pair[0] - min_x, pair[1] - min_y) for pair in dig_history}

    def __str__(self) -> str:
        board_as_str = []
        for i in range(self.rows_count):
            this_row = []
            for j in range(self.columns_count):
                if (i, j) in self.board:
                    this_row.append("#")
                else:
                    this_row.append(".")
            board_as_str.append("".join(this_row))

        return "\n".join(board_as_str)

    def check_if_point_surrounded(self, position_x: int, position_y: int) -> bool:
        """
        Given the board, and a starting position, perform a depth first search to determine
        if the given point is surrounded.
        """
        # To prevent cycles, keep track of where we have traversed.
        spots_already_visited = set()
        spots_to_visit = [(position_x, position_y)]
        while len(spots_to_visit) > 0:
            current_position = spots_to_visit.pop()
            if current_position in spots_already_visited:
                continue
            spots_already_visited.add(current_position)
            (current_pos_x, current_pos_y) = (current_position)
            # If we successfully went of the board, then we know with certainty
            # that the point in question is not surrounded.
            if current_pos_x < 0 or current_pos_y < 0:
                return False
            try:
                board_position = "." if (
                    current_pos_x, current_pos_y) not in self.board else "#"
                if board_position == ".":
                    # If this board position is a '.', then we need to explore
                    # all four surrounding squares.
                    spots_to_visit.append((current_pos_x, current_pos_y + 1))
                    spots_to_visit.append((current_pos_x, current_pos_y - 1))
                    spots_to_visit.append((current_pos_x + 1, current_pos_y))
                    spots_to_visit.append((current_pos_x - 1, current_pos_y))
            except IndexError:
                # Same logic as above. If off the board, then we know we cannot be surrounded.
                return False
        return True

    def find_surrounded_point(self) -> tuple[int, int]:
        for row_number in range(self.rows_count):
            lowest_column_number = 0
            for col_number in range(self.columns_count):
                if (row_number, col_number) not in self.board:
                    lowest_column_number = col_number
                    break
            highest_column_number = 0
            for col_number in range(self.columns_count - 1, 0, -1):
                if (row_number, col_number) not in self.board:
                    highest_column_number = col_number
                    break
            for position_y in range(lowest_column_number, highest_column_number):
                if (row_number, position_y) not in self.board and self.check_if_point_surrounded(row_number, position_y):
                    return (row_number, position_y)

        raise RuntimeError("The given board has zero surrounded points!")

    def dig_out_board(self) -> set[tuple[int, int]]:
        surrounded_point = self.find_surrounded_point()
        print(f"Got surrounded point: {surrounded_point}")
        spots_already_visited = set()
        spots_to_visit = [surrounded_point]
        while len(spots_to_visit) > 0:
            current_position = spots_to_visit.pop()
            if current_position in spots_already_visited:
                continue
            spots_already_visited.add(current_position)
            (current_pos_x, current_pos_y) = (current_position)
            # If we successfully went of the board, then we know with certainty
            # that the point in question is not surrounded.
            if current_pos_x < 0 or current_pos_y < 0:
                continue
            try:
                board_position = "." if (
                    current_pos_x, current_pos_y) not in self.board else "#"
                if board_position == ".":
                    self.board.add((current_pos_x, current_pos_y))
                    # If this board position is a '.', then we need to explore
                    # all four surrounding squares.
                    spots_to_visit.append((current_pos_x, current_pos_y + 1))
                    spots_to_visit.append((current_pos_x, current_pos_y - 1))
                    spots_to_visit.append((current_pos_x + 1, current_pos_y))
                    spots_to_visit.append((current_pos_x - 1, current_pos_y))
            except IndexError:
                # Same logic as above. If off the board, then we know we cannot be surrounded.
                continue
        return self.board


class SolveDay18(SolveAdvent):
    def solve_part1(self):
        parsed_dig_directions = process_input_file_part1(self.file_content)
        dig_board = Board(dig_directions=parsed_dig_directions).dig_out_board()
        print(f"Total Spots Dug Out: {len(dig_board)}")

    def solve_part2(self):
        parsed_dig_directions = process_input_file_part2(self.file_content)
        dig_board = Board(dig_directions=parsed_dig_directions).dig_out_board()
        print(f"Total Spots Dug Out: {len(dig_board)}")
