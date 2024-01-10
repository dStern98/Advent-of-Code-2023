from .base import SolveAdvent


class SolveDay18(SolveAdvent):
    def solve_part1(self):
        parsed_dig_directions = process_input_file_part1(self.file_content)
        dig_history = build_dig_history(parsed_dig_directions)
        dig_board = draw_dig_map(dig_history)
        surrounded_point = find_surrounded_point(dig_board)
        dig_board = dig_out_board(surrounded_point, dig_board)
        total_dug_spots = sum(
            len([spot for spot in row if spot == "#"])
            for row in dig_board
        )
        print(f"Total Spots Dug Out: {total_dug_spots}")

    def solve_part2(self):
        parsed_dig_directions = process_input_file_part2(self.file_content)
        dig_history = build_dig_history(parsed_dig_directions)
        dig_board = draw_dig_map(dig_history)
        surrounded_point = find_surrounded_point(dig_board)
        dig_board = dig_out_board(surrounded_point, dig_board)
        total_dug_spots = sum(
            len([spot for spot in row if spot == "#"])
            for row in dig_board
        )
        print(f"Total Spots Dug Out: {total_dug_spots}")


def process_input_file_part1(file_content: list[str]) -> list[tuple[str, int]]:
    parsed_dig_directions = []
    for line in file_content:
        (dig_direction, dig_distance, _) = line.split(" ")
        dig_direction = dig_direction.strip()
        dig_distance = int(dig_distance.strip())
        parsed_dig_directions.append((dig_direction, dig_distance))

    return parsed_dig_directions


DIG_DIRECTION_MAP = {
    "0": "R",
    "1": "D",
    "2": "L",
    "3": "U"
}


def process_input_file_part2(file_content: list[str]) -> list[tuple[str, int]]:
    parsed_dig_directions = []
    for line in file_content:
        (_, _, dig_color) = line.split(" ")
        dig_color = dig_color.strip().removeprefix("(#").removesuffix(")")
        dig_direction = DIG_DIRECTION_MAP[dig_color[-1]]
        dig_distance = int(dig_color[0:len(dig_color) - 1], 16)
        parsed_dig_directions.append((dig_direction, dig_distance))

    return parsed_dig_directions


def build_dig_history(dig_instructions: list[tuple[str, int]]) -> list[tuple[int, int]]:
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
    return [(pair[0] - min_x, pair[1] - min_y) for pair in dig_history]


def draw_dig_map(dig_history: list[tuple[int, int]]) -> list[list[str]]:
    rows_count = max((pair[0] for pair in dig_history))
    columns_count = max(((pair[1] for pair in dig_history)))
    dig_board = [
        ["." for _ in range(columns_count + 1)]
        for _ in range(rows_count + 1)
    ]
    for dug_spot in dig_history:
        (dug_pos_x, dug_pos_y) = dug_spot
        dig_board[dug_pos_x][dug_pos_y] = "#"
    return dig_board


def check_if_point_surrounded(board: list[list[str]], position_x: int, position_y: int) -> bool:
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
            board_position = board[current_pos_x][current_pos_y]
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


def find_surrounded_point(board: list[list[str]]) -> tuple[int, int]:
    for position_x in range(len(board)):
        for position_y in range(len(board[0])):
            if board[position_x][position_y] == "." and check_if_point_surrounded(board, position_x, position_y):
                return (position_x, position_y)

    raise RuntimeError("The given board has zero surrounded points!")


def dig_out_board(surrounded_point: tuple[int, int], board: list[list[str]]):
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
            board_position = board[current_pos_x][current_pos_y]
            if board_position == ".":
                board[current_pos_x][current_pos_y] = "#"
                # If this board position is a '.', then we need to explore
                # all four surrounding squares.
                spots_to_visit.append((current_pos_x, current_pos_y + 1))
                spots_to_visit.append((current_pos_x, current_pos_y - 1))
                spots_to_visit.append((current_pos_x + 1, current_pos_y))
                spots_to_visit.append((current_pos_x - 1, current_pos_y))
        except IndexError:
            # Same logic as above. If off the board, then we know we cannot be surrounded.
            continue
    return board
