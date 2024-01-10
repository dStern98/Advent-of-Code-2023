from abc import ABC, abstractmethod


class SolveAdvent(ABC):
    """
    Abstract base class for all python solutions to 
    a days Advent Calendar problems. Any days solution
    should inherit from this class and implement the required
    methods.
    """

    def __init__(self, file_path: str):
        with open(file_path, "r") as file:
            file_content = file.readlines()
        self.file_content = file_content

    @abstractmethod
    def solve_part1(self):
        """
        Solve Part1 of the Days Advent Problem
        """
        pass

    @abstractmethod
    def solve_part2(self):
        """
        Solve Part2 of the Days Advent Problem.
        """
        pass
