from functools import reduce
import operator
import collections
from itertools import chain, product
import string
from typing import Dict, List, Tuple


def get_input():
    parsed = map(lambda s: s.strip().split(","), open("full_input.txt"))

    inted = [list(map(lambda i: int(i), l)) for l in parsed]

    # return [[1, 1, 1], [2, 1, 1]]
    return inted


def calculate_part1(puzzle_input):
    print("Calculating for", list(puzzle_input))

    space = collections.defaultdict(lambda: False)

    # Fill space with voxels
    for line in puzzle_input:
        space[tuple(line)] = True

    def get_neighbor(voxel, offset):
        return (voxel[0] + offset[0], voxel[1] + offset[1], voxel[2] + offset[2])

    def free_count(voxel, space):
        offsets = [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)]

        free = 0
        for offset in offsets:
            neighbor = get_neighbor(voxel, offset)
            if not space.get(neighbor):
                free += 1
        return free

    free_for_each = [free_count(line, space) for line in puzzle_input]

    return sum(free_for_each)


def calculate_part2(puzzle_input):
    print("Calculating for", list(puzzle_input))

    occupied: Dict[Tuple(int, int, int), bool] = collections.defaultdict(lambda: False)

    min_x, min_y, min_z = -50000, -50000, -50000
    max_x, max_y, max_z = 50000, 50000, 50000

    # Fill space with voxels
    for line in puzzle_input:
        occupied[tuple(line)] = True

        min_x, max_x = min([l[0] for l in puzzle_input]), max(
            [l[0] for l in puzzle_input]
        )
        min_y, max_y = min([l[1] for l in puzzle_input]), max(
            [l[1] for l in puzzle_input]
        )
        min_z, max_z = min([l[2] for l in puzzle_input]), max(
            [l[2] for l in puzzle_input]
        )

    non_enclosed_free_space = collections.defaultdict(lambda: False)
    enclosed_free_space = collections.defaultdict(lambda: False)

    print(
        f"X range: {min_x}-{max_x}, Y range: {min_y}-{max_y}, Z ragne: {min_z}-{max_z}"
    )

    def get_neighbor_offset(voxel, offset):
        return (voxel[0] + offset[0], voxel[1] + offset[1], voxel[2] + offset[2])

    def get_neighbors(voxel):
        offsets = [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)]

        for offset in offsets:
            yield (get_neighbor_offset(voxel, offset))

    def test_free_space(point, occupied):
        stack = collections.deque()
        stack.append(point)

        visited = []

        free_space = False

        print(f"Flood fill for {point}")

        while stack:
            # print(stack)
            el = stack.pop()
            visited.append(el)

            for neighbor in get_neighbors(el):
                if neighbor not in visited and not occupied.get(neighbor):
                    if (
                        neighbor[0] < min_x - 1
                        or neighbor[0] > max_x + 1
                        or neighbor[1] < min_y - 1
                        or neighbor[1] > max_y + 1
                        or neighbor[2] < min_z - 1
                        or neighbor[2] > max_z + 1
                    ) or tuple(neighbor) in non_enclosed_free_space:
                        free_space = True
                        # Don't append it to the stack to prevent infinite expansion
                        # Also don't append it to the stack if we have already booked this as part of the free envelope
                    else:
                        stack.append(neighbor)

        # Made it through the fill
        for v in visited:
            if free_space:
                non_enclosed_free_space[tuple(v)] = True
            else:
                enclosed_free_space[tuple(v)] = True
        return free_space

    def free_count(voxel, space):
        # print("Testing free for ", voxel)

        free = 0

        for neighbor in get_neighbors(voxel):
            if not space.get(neighbor) and test_free_space(neighbor, occupied):
                free += 1

        # print(f"Free: {free}")

        return free

    free_for_each = [free_count(line, occupied) for line in puzzle_input]

    return sum(free_for_each)


def part1(puzzle_input):
    return calculate_part1(puzzle_input)


def part2(puzzle_input):
    return calculate_part2(puzzle_input)


# def test_calculation():
# assert calculate_part1(">") == false

# assert calculate_part2("^v") == false


if __name__ == "__main__":
    puzzle_input = get_input()

    # print("Part 1 answer:", part1(puzzle_input))
    print("Part 2 answer:", part2(puzzle_input))
