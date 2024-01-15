from .base import SolveAdvent
import time


class SolveDay12(SolveAdvent):
    def solve_part1(self):
        total_partition_count = 0
        t1 = time.perf_counter()
        for line in self.file_content:
            (parts, group_pattern) = line.split(" ")
            group_pattern_parsed = [int(group_count)
                                    for group_count in group_pattern.split(',')]
            total_partition_count += compute_possible_partitions(
                parts.strip(), group_pattern_parsed)
        print(
            f"Full processing completed in {time.perf_counter() - t1} seconds")

        print(f"Total Possible Part Partitions is: {total_partition_count}")

    def solve_part2(self):
        ...
        total_partition_count = 0
        for line in self.file_content:
            (parts, group_pattern) = line.split(" ")
            group_pattern_parsed = [int(group_count)
                                    for group_count in group_pattern.split(',')]
            parts = f"{parts.strip()}?" * 5
            parts = parts.removesuffix("?")
            group_pattern_parsed = group_pattern_parsed * 5
            total_partition_count += compute_possible_partitions(
                parts, group_pattern_parsed)

        print(f"Total Possible Part Partitions is: {total_partition_count}")


def group_matches_requirements(
        parts: str,
        required_part_partitions: list[int]) -> bool:
    """
    Determine conclusively if a possible springs partition
    of damaged and not damaged parts matches the required partitions.
    """
    observed_part_partitions = [len(part_group)
                                for part_group in parts.split(".") if part_group]
    if len(observed_part_partitions) != len(required_part_partitions):
        return False

    for observed_part_partition, required_part_partition in zip(observed_part_partitions, required_part_partitions):
        if observed_part_partition != required_part_partition:
            return False

    return True


def compute_possible_partitions(
        parts: str,
        required_part_partitions: list[int]) -> int:
    if "?" not in parts:
        if group_matches_requirements(parts, required_part_partitions):
            return 1
        return 0

    viable_partition_count = 0
    for potential_symbol in (".", "#"):
        part_replaced = parts.replace("?", potential_symbol, 1)
        viable_partition_count += compute_possible_partitions(
            part_replaced, required_part_partitions)
    return viable_partition_count
