from functools import reduce
import operator
import collections
from itertools import chain
from typing import List, Tuple

COLS = 6
ROWS = 4

CoordList = List[Tuple[int, int]]


def occupied_spaces(
    northers: CoordList,
    easters: CoordList,
    southers: CoordList,
    westers: CoordList,
):
    return set([c for c in northers + easters + southers + westers])


def step_northers(northers: CoordList) -> CoordList:
    return [(x, y - 1 if y - 1 >= 0 else ROWS - 1) for (x, y) in northers]


def step_southers(southers: CoordList) -> CoordList:
    return [(x, y + 1 if y + 1 < ROWS else 0) for (x, y) in southers]


def step_westers(westers: CoordList) -> CoordList:
    return [(x - 1 if x - 1 >= 0 else COLS - 1, y) for (x, y) in westers]


def step_easters(easters: CoordList) -> CoordList:
    return [(x + 1 if x + 1 < COLS else 0, y) for (x, y) in easters]


def step_all(
    northers: CoordList,
    easters: CoordList,
    southers: CoordList,
    westers: CoordList,
) -> Tuple[CoordList, CoordList, CoordList, CoordList]:
    return (
        step_northers(northers),
        step_easters(easters),
        step_southers(southers),
        step_westers(westers),
    )


def print_map(
    northers: CoordList, easters: CoordList, southers: CoordList, westers: CoordList
):
    for j in range(ROWS):
        for i in range(COLS):
            if (i, j) in northers:
                print("^", end="")
            elif (i, j) in easters:
                print(">", end="")
            elif (i, j) in southers:
                print("v", end="")
            elif (i, j) in westers:
                print("<", end="")
            else:
                print(".", end="")
        print()


def get_input() -> Tuple[CoordList, CoordList, CoordList, CoordList]:
    i = open("test_input.txt")

    northers, easters, southers, westers = [], [], [], []
    for j, l in enumerate(list(i)[1:-1]):
        for i, c in enumerate(l[1:-2]):
            if c == ">":
                easters.append((i, j))
            elif c == "<":
                westers.append((i, j))
            elif c == "^":
                northers.append((i, j))
            elif c == "v":
                southers.append((i, j))

    # interior = list(map(lambda l: l[1:-2], list(i)[1:-1]))
    return northers, easters, southers, westers


def calculate_part1(puzzle_input):
    print("Calculating for", puzzle_input)

    northers, easters, southers, westers = puzzle_input

    # o = occupied_spaces(northers, easters, southers, westers)

    print("Start")
    print_map(northers, easters, southers, westers)

    northers, easters, southers, westers = step_all(
        northers, easters, southers, westers
    )

    print()
    print("Round 1")
    print_map(northers, easters, southers, westers)

    northers, easters, southers, westers = step_all(
        northers, easters, southers, westers
    )

    print()
    print("Round 2")
    print_map(northers, easters, southers, westers)

    return 1


def calculate_part2(puzzle_input):
    print("Calculating", puzzle_input)

    return 1


def part1(puzzle_input):
    return calculate_part1(puzzle_input)


def part2(puzzle_input):
    return calculate_part2(puzzle_input)


def test_calculation():
    assert calculate_part1(">") == false

    # assert calculate_part2("^v") == false


if __name__ == "__main__":
    puzzle_input = get_input()

    print("Part 1 answer:", part1(puzzle_input))
    # print("Part 2 answer:", part2(puzzle_input))
