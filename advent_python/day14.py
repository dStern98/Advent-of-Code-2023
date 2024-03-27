from .base import SolveAdvent


class SolveDay14(SolveAdvent):
    def solve_part1(self):
        rock_slider = RockSlider(self.file_content)
        # rock_slider.pprint_map()
        # rock_slider.slide_row_rocks_north(rock_slider.transposed_map[2])
        rock_slider.slide_all_rocks_north()

    def solve_part2(self):
        ...


class RockSlider:
    def __init__(self, file_content: list[str]):
        self.map = [[char for char in row if char != "\n"]
                    for row in file_content]

    def transpose_map(self):
        transposed_content = [["" for _ in range(
            len(self.map))] for _ in range(len(self.map[0]))]
        for row_number in range(len(self.map)):
            for col_number in range(len(self.map[0])):
                transposed_content[col_number][row_number] = self.map[row_number][col_number]
        self.map = transposed_content

    def pprint_map(self):
        print("\n".join(("".join(row) for row in self.map)))

    @staticmethod
    def slide_row_rocks_north(row: list[str]):
        # copied_row = [item for item in row]
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
        # print("------------------------------")
        # print(f"Row Before: {copied_row}")
        # print(f"Row after: {row}")

    def slide_all_rocks_north(self):
        """
        ...
        """
        self.transpose_map()
        # self.pprint_map()
        for row in self.map:
            self.slide_row_rocks_north(row)
        self.transpose_map()
        # self.pprint_map()

        total_load = 0
        platform_height = len(self.map)
        for index, row in enumerate(self.map):
            score_per = platform_height - index
            number_of_round_rocks = len([item for item in row if item == "O"])
            total_load += number_of_round_rocks * score_per

        print(f"Total Load on North Platform is {total_load}")
