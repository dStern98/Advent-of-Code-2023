from .base import SolveAdvent
from collections import deque
from dataclasses import dataclass


@dataclass
class TrailExplorer:
    row: int
    col: int
    visited: set[tuple[int, int]]
    distance_traveled: int

    def off_the_map(self, trail_map: list[list[str]]) -> bool:
        """
        Whether or not the given `TrailExplorer` instance is off the trail_map.
        """
        if self.row < 0 or self.col < 0 or self.row > len(trail_map) - 1 or self.col > len(trail_map[0]) - 1:
            return True
        return False

    def target_reached(self, target_row: int, target_col: int) -> bool:
        return self.row == target_row and self.col == target_col

    def visit_position(self):
        """
        Add the current row, col pair to the visited set.
        """
        self.visited.add((self.row, self.col))

    def in_cycle(self) -> bool:
        """
        Returns whether or not the Explorer has completed a cycle.
        """
        return (self.row, self.col) in self.visited

    def up_one(self):
        return TrailExplorer(
            row=self.row - 1,
            col=self.col,
            visited=self.visited.copy(),
            distance_traveled=self.distance_traveled + 1
        )

    def down_one(self):
        return TrailExplorer(
            row=self.row + 1,
            col=self.col,
            visited=self.visited.copy(),
            distance_traveled=self.distance_traveled + 1
        )

    def left_one(self):
        return TrailExplorer(
            row=self.row,
            col=self.col - 1,
            visited=self.visited.copy(),
            distance_traveled=self.distance_traveled + 1
        )

    def right_one(self):
        return TrailExplorer(
            row=self.row,
            col=self.col + 1,
            visited=self.visited.copy(),
            distance_traveled=self.distance_traveled + 1
        )

    def draw_traversed_path(self, hiking_trail_map: list[list[str]]):
        """
        Helper method for debugging, draws the trail map populated
        with the traversal path of the explorer.
        """
        trail_map_copied = [[copied_char for copied_char in line]
                            for line in hiking_trail_map]
        for (row, col) in self.visited:
            trail_map_copied[row][col] = "O"
        map = '\n'.join(["".join(row) for row in trail_map_copied])
        print(f"_----------------------------------")
        print(
            f"Explorer reached target with distance traveled: {self.distance_traveled}")
        print(f"Explorer visited len {len(self.visited)}")
        print(f"{map}")

    def generate_next_moves(
            self,
            trail_map: list[list[str]],
            count_slopes_as_trails: bool) -> list["TrailExplorer"]:
        """
        Depending on whether the current position is a trail, slope, or forrest, 
        return the next allowed moves from the current position.
        """
        current_position = trail_map[self.row][self.col]
        if current_position == "#":
            return []
        self.visit_position()
        if current_position == "#":
            next_moves = []
        elif current_position == "." or count_slopes_as_trails:
            next_moves = [
                self.up_one(),
                self.down_one(),
                self.left_one(),
                self.right_one()
            ]
        elif current_position == "^":
            next_moves = [self.up_one()]
        elif current_position == ">":
            next_moves = [self.right_one()]
        elif current_position == "<":
            next_moves = [self.left_one()]
        elif current_position == "v":
            next_moves = [self.down_one()]
        return [explorer for explorer in next_moves if not explorer.off_the_map(trail_map)]


class SolveDay23(SolveAdvent):
    def find_longest_hike(
            self,
            allow_free_slope_traversal: bool):
        """
        Find the longest possible hike that does not repeat any part of the trail.
        """
        hiking_trail_map = [[char for char in line if char != "\n"]
                            for line in self.file_content]
        (target_row, target_column) = (
            len(hiking_trail_map) - 1, hiking_trail_map[-1].index("."))
        # Use a deque for breadth first search
        queue = deque(
            (TrailExplorer(
                row=0,
                col=hiking_trail_map[0].index("."),
                distance_traveled=0,
                visited=set()
            ),
            )
        )

        maximum_distance_traveled = 0
        while len(queue) > 0:
            current_explorer = queue.popleft()
            if current_explorer.in_cycle():
                continue
            if current_explorer.target_reached(target_row, target_column):
                if current_explorer.distance_traveled > maximum_distance_traveled:
                    maximum_distance_traveled = current_explorer.distance_traveled
                continue
            queue.extend(current_explorer.generate_next_moves(
                hiking_trail_map, allow_free_slope_traversal))
        print(
            f"After iteration, maximum possible hiking distance is {maximum_distance_traveled}")

    def solve_part1(self):
        self.find_longest_hike(allow_free_slope_traversal=False)

    def solve_part2(self):
        self.find_longest_hike(allow_free_slope_traversal=True)
