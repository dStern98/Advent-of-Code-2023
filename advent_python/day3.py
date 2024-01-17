from .base import SolveAdvent
from dataclasses import dataclass


@dataclass
class PotentialPartNumber:
    """
    Represents the row, starting col, and ending col
    of a potential part number, as well as a string version
    of the number itself.
    """
    number: str
    row: int
    col_start: int
    col_end: int

    def is_part_number(self, board: list[str]) -> bool:
        """
        Determine whether a potential part number is actually a part number,
        which requires that there be an adjacent symbol to the number in any direction.
        """
        # First, check left to see if its a symbol other than '.'
        if self.col_start > 0 and board[self.row][self.col_start - 1] != ".":
            return True
        # Then check right for the same
        if self.col_end < len(board[0]) - 1 and board[self.row][self.col_end + 1] != ".":
            return True
        # Check the entire row above the number for a symbol, including diagonal
        left_boundary = max((self.col_start - 1, 0))
        right_boundary = min((self.col_end + 2, len(board[0])))
        if self.row > 0:
            for symbol in board[self.row - 1][left_boundary: right_boundary]:
                if symbol != "." and not symbol.isdigit():
                    return True
        # Check the row below the number for a symbol
        if self.row < len(board)-1:
            for symbol in board[self.row+1][left_boundary: right_boundary]:
                if symbol != "." and not symbol.isdigit():
                    return True
        return False


@dataclass
class PotentialGear:
    """
    Represents the row, col position of a potential gear, represented
    by the '*' symbol.
    """
    row: int
    col: int

    def is_gear(
            self,
            embedded_numbers_map: dict[int, list[PotentialPartNumber]]
    ) -> int:
        """
        Given a map of embedded numbers, determine if a potential gear is an actual gear, 
        which requires that it have exactly two adjacent numbers to it.
        Returns the gear ratio of the gear if it indeed a gear.
        """
        # First, check left & right for adjacent embedded numbers
        left_numbers = [embedded_number for embedded_number in embedded_numbers_map[self.row]
                        if embedded_number.col_end == self.col - 1] if self.row in embedded_numbers_map else []
        right_numbers = [embedded_number for embedded_number in embedded_numbers_map[self.row]
                         if embedded_number.col_start == self.col + 1] if self.row in embedded_numbers_map else []

        # Check above and below for adjacent numbers
        upper_numbers = [embedded_number for embedded_number in embedded_numbers_map[self.row - 1] if set(range(
            self.col-1, self.col + 2)).intersection(set(range(embedded_number.col_start, embedded_number.col_end + 1)))] if self.row - 1 in embedded_numbers_map else []
        lower_numbers = [embedded_number for embedded_number in embedded_numbers_map[self.row + 1] if set(range(
            self.col-1, self.col + 2)).intersection(set(range(embedded_number.col_start, embedded_number.col_end + 1)))] if self.row + 1 in embedded_numbers_map else []

        adjacent_numbers = [
            *left_numbers,
            *right_numbers,
            *lower_numbers,
            *upper_numbers
        ]
        if len(adjacent_numbers) == 2:
            gear1, gear2 = adjacent_numbers
            return int(gear1.number) * int(gear2.number)
        return 0


class SolveDay3(SolveAdvent):
    def solve_part1(self):
        total_parts_number = 0
        board = [line.strip() for line in self.file_content]
        for row, line in enumerate(self.file_content):
            col_numbers = []
            latest_number = ""
            for column, symbol in enumerate(line):
                if symbol.isdigit():
                    latest_number += symbol
                    col_numbers.append(column)
                elif latest_number:
                    potential_part_number = PotentialPartNumber(
                        number=latest_number,
                        row=row,
                        col_start=min(col_numbers),
                        col_end=max(col_numbers)
                    )
                    if potential_part_number.is_part_number(board):
                        total_parts_number += int(latest_number)
                    latest_number = ""
                    col_numbers = []
        print(f"Final Parts Sum is {total_parts_number}")

    def solve_part2(self):
        possible_gears: list[PotentialGear] = []
        embedded_numbers_map: dict[int, list[PotentialPartNumber]] = {}
        board = [line.strip() for line in self.file_content]
        # Step 1: Walk the board and record all possible gears and all actual part numbers.
        for row, line in enumerate(self.file_content):
            col_numbers = []
            latest_number = ""
            for column, symbol in enumerate(line):
                if symbol == "*":
                    possible_gears.append(PotentialGear(
                        row=row,
                        col=column
                    ))
                if symbol.isdigit():
                    latest_number += symbol
                    col_numbers.append(column)
                elif latest_number:
                    potential_part_number = PotentialPartNumber(
                        number=latest_number,
                        row=row,
                        col_start=min(col_numbers),
                        col_end=max(col_numbers)
                    )
                    if potential_part_number.is_part_number(board):
                        embedded_numbers_map.setdefault(
                            row, []).append(potential_part_number)
                    latest_number = ""
                    col_numbers = []
        # For each potential gear, check if they are a gear, and if so, record the
        # gear ratio.
        total_gear_ratios = 0
        for possible_gear in possible_gears:
            total_gear_ratios += possible_gear.is_gear(embedded_numbers_map)
        print(f"Total Gear Ratio is {total_gear_ratios}")
