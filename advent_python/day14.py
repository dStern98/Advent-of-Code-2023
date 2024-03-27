from .base import SolveAdvent


class SolveDay14(SolveAdvent):
    def solve_part1(self):
        RockSlider(self.file_content).slide_all_rocks_north()

    def solve_part2(self):
        # The current implementation is far too inefficient to have
        # any hope of completing for the desired trillion iterations.
        RockSlider(self.file_content).spin_cycle(1_000_000_000)


class RockSlider:
    def __init__(self, file_content: list[str]):
        """
        :param map: the platform of rocks as described in the platform.
        """
        self.map = [[char for char in row if char != "\n"]
                    for row in file_content]

    @property
    def north_support_load(self) -> int:
        """
        The total load on the north support beam based
        on the current `map`
        """
        total_load = 0
        platform_height = len(self.map)
        for index, row in enumerate(self.map):
            score_per = platform_height - index
            number_of_round_rocks = len([item for item in row if item == "O"])
            total_load += number_of_round_rocks * score_per
        return total_load

    def flip_map_180(self):
        for row in self.map:
            row.reverse()

    def transpose_map(self):
        """
        Perform a transpose opperation on the map, so that rows and columns flip.
        """
        transposed_content = [["" for _ in range(
            len(self.map))] for _ in range(len(self.map[0]))]
        for row_number in range(len(self.map)):
            for col_number in range(len(self.map[0])):
                transposed_content[col_number][row_number] = self.map[row_number][col_number]
        self.map = transposed_content

    def pprint_map(self):
        """
        Print the map as its shown in the problem prompt.
        """
        print("\n".join(("".join(row) for row in self.map)))

    @staticmethod
    def slide_row_rocks_west(row: list[str]):
        """
        In place operation to slide all rocks west as
        far as they will go. Only circular rocks `O` slide,
        cube-shaped rocks `#` do not slide.
        """
        for current_position, rock in enumerate(row):
            if rock != ".":
                continue
            explorer_index = current_position + 1
            while explorer_index < len(row):
                next_rock = row[explorer_index]
                if next_rock == "O":
                    row[current_position] = "O"
                    row[explorer_index] = "."
                    break
                elif next_rock == "#":
                    break
                explorer_index += 1

    def slide_all_rocks_west(self):
        for row in self.map:
            self.slide_row_rocks_west(row)

    def slide_all_rocks_north(self):
        """
        Use the `slide_all_rocks_west` method, but transpose
        the map before and after sliding the rocks west. The end result
        is equivalent to sliding all of the rocks North, albeit
        not optimally
        """
        self.transpose_map()
        self.slide_all_rocks_west()
        self.transpose_map()
        print(f"Total Load on North Platform is {self.north_support_load}")

    def spin_cycle(self, number_of_cycles: int):
        """
        As directed in Part2, Spin the map by first moving
        North, West, South, and East.

        The current implementation is extremely inefficient because
        of all of the tranposes and 180 operations. A better version
        would directly implement the sliding logic for all 4 directions,
        """
        for cycle in range(number_of_cycles):
            if cycle % 1_000 == 0:
                print(f"Reached cycle {cycle}/{number_of_cycles}")
            # First, roll all of the rocks North
            self.transpose_map()
            self.slide_all_rocks_west()
            self.transpose_map()

            # Now roll west
            self.slide_all_rocks_west()

            # Roll South
            self.transpose_map()
            self.flip_map_180()
            self.slide_all_rocks_west()
            self.flip_map_180()
            self.transpose_map()

            # Roll East
            self.flip_map_180()
            self.slide_all_rocks_west()
            self.flip_map_180()
        print(f"Load on North Support Beam: {self.north_support_load}")
