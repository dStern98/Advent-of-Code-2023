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
            # parts = f"{parts.strip()}?" * 5
            # parts = parts.removesuffix("?")
            # group_pattern_parsed = group_pattern_parsed * 5
            partition_count = compute_possible_partitions(
                parts, group_pattern_parsed)
            print(
                f"Parts {parts} has a floor of {partition_count ** 5} possible partitions")

        print(f"Total Possible Part Partitions is: {total_partition_count}")


def spring_arrangement_is_ok(
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


def branch_is_impossible(
        parts: str,
        required_part_partitions: list[int]) -> bool:
    """
    Because the recursion mutates the ?'s from left to right, 
    any branch component that has no question marks is final, so as soon
    as we find a component without a ? that does not match the required_part_partition, 
    we know that this branch cannot end up working out.
    """
    observed_part_partitions = (part_group
                                for part_group in parts.split(".") if part_group)
    for (observed_part, required_part_partition) in zip(observed_part_partitions, required_part_partitions):
        if "?" in observed_part:
            break
        if len(observed_part) != required_part_partition:
            return True
    return False


def compute_possible_partitions(
        parts: str,
        required_part_partitions: list[int]) -> int:
    """
    Recursively explore the possible partitions, by filling
    in the question marks one at a time. Return as soon as either
    there are no questions marks or we can conclusively conclude
    that the given path cannot possibly get a correct answer.
    """
    # If ? is not in the parts string, then we can conclusively determine whether or not
    # the given branch solves the given partition requirements.
    if "?" not in parts:
        if spring_arrangement_is_ok(parts, required_part_partitions):
            return 1
        return 0
    # Check if an early return is possible
    if branch_is_impossible(parts, required_part_partitions):
        return 0

    # Otherwise, recursively invoke the function, replacing a single ?
    # with one of the other two possible symbols
    viable_partition_count = 0
    for potential_symbol in (".", "#"):
        part_replaced = parts.replace("?", potential_symbol, 1)
        viable_partition_count += compute_possible_partitions(
            part_replaced, required_part_partitions)
    return viable_partition_count
