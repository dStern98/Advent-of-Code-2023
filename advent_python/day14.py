from .base import SolveAdvent


class SolveDay14(SolveAdvent):
    def solve_part1(self):
        RockSlider(self.file_content).slide_all_rocks_north()

    def solve_part2(self):
        cycles_to_perform = 1_000_000_000
        cycle_info = RockSlider(
            self.file_content
        ).spin_cycle(
            cycles_to_perform,
            break_on_cycle=True
        )
        if isinstance(cycle_info, tuple):
            cycle_start, cycle_end = cycle_info
            # We know the cycle repeats every cycle_delta iterations
            cycle_delta = cycle_end - cycle_start
            optimized_cycles_to_perform = cycles_to_perform % cycle_delta
            while optimized_cycles_to_perform < cycle_start:
                optimized_cycles_to_perform += cycle_delta
            # The optimized cycle count should have the map in an equivalent state to the original cycles_to_perform
            print(
                f"Optimized cycle count {optimized_cycles_to_perform} should be equivalent to {cycles_to_perform}")
            cycle_info = RockSlider(
                self.file_content).spin_cycle(optimized_cycles_to_perform, False)


class RockSlider:
    def __init__(self, file_content: list[str]):
        """
        :param map: the platform of rocks as described in the platform.
        """
        self.map_to_mutate = [[char for char in row if char != "\n"]
                              for row in file_content]

    @property
    def north_support_load(self) -> int:
        """
        The total load on the north support beam based
        on the current `map`
        """
        total_load = 0
        platform_height = len(self.map_to_mutate)
        for index, row in enumerate(self.map_to_mutate):
            score_per = platform_height - index
            number_of_round_rocks = len([item for item in row if item == "O"])
            total_load += number_of_round_rocks * score_per
        return total_load

    def flip_map_180(self):
        for row in self.map_to_mutate:
            row.reverse()

    def transpose_map(self):
        """
        Perform a transpose opperation on the map, so that rows and columns flip.
        """
        transposed_content = [["" for _ in range(
            len(self.map_to_mutate))] for _ in range(len(self.map_to_mutate[0]))]
        for row_number in range(len(self.map_to_mutate)):
            for col_number in range(len(self.map_to_mutate[0])):
                transposed_content[col_number][row_number] = self.map_to_mutate[row_number][col_number]
        self.map_to_mutate = transposed_content

    @property
    def display_map(self) -> str:
        """
        Print the map as its shown in the problem prompt.
        """
        return "\n".join(("".join(row) for row in self.map_to_mutate))

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
        for row in self.map_to_mutate:
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

    def cycle_once(self):
        """
        Perform one spin cycle as explained in Part2 of the problem.
        """
        # Roll North
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

    def spin_cycle(
            self,
            requested_cycles: int,
            break_on_cycle: bool) -> tuple[int, int] | None:
        """
        As directed in Part2, spin the map `requsted_cycles` times.

        A key optimization is to look for the occurence of cycles. Because
        the map changes over time is deterministic, once a cycle occurs it will infinitely
        repeat. If `break_on_cycle=True`, then as soon as a cycle is detected, the loop will break,
        returning a tuple of `(lower_cycle_number, upper_cycle_number)`
        """
        cycle_tracker: dict[str, int] = {}
        for cycle_number in range(requested_cycles):
            self.cycle_once()
            if break_on_cycle:
                if self.display_map in cycle_tracker:
                    lower_cycle_number, upper_cycle_number = cycle_tracker[
                        self.display_map], cycle_number
                    # If a cycle is found and break_on_cycle is True,
                    # then return early with cycle lower, upper bound if found.
                    return lower_cycle_number, upper_cycle_number
                # If the current map is not in the cycle_tracker, then log it
                cycle_tracker[self.display_map] = cycle_number
        print(
            f"Load on north beam after {requested_cycles} cycles is {self.north_support_load}")
