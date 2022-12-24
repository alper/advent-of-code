from functools import reduce
import operator
import collections
from itertools import chain
from typing import List, Tuple

# COLS = 6
# ROWS = 4

COLS = 120
ROWS = 25

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


def get_next_positions(pos: Tuple[int, int]) -> List[Tuple[int, int]]:
    n = []

    # Staying put is also a potential position
    n.append((pos[0], pos[1]))

    if pos[0] > 0:
        n.append((pos[0] - 1, pos[1]))

    if pos[0] < COLS - 1:
        n.append((pos[0] + 1, pos[1]))

    if pos[1] > 0:
        n.append((pos[0], pos[1] - 1))

    if pos[1] < ROWS - 1:
        n.append((pos[0], pos[1] + 1))

    return n


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
    # i = open("test_input.txt")
    i = open("full_input.txt")

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

    blizzards = step_all(northers, easters, southers, westers)

    pos = (0, 0)

    minutes = 1

    candidate_state = collections.deque()
    seen = set()

    candidate_state.append((pos, minutes, blizzards))

    while True:
        pos, minutes, blizzards = candidate_state.popleft()
        print(f"In minute: {minutes}, position: {pos} seen size: {len(seen)}")
        print_map(blizzards[0], blizzards[1], blizzards[2], blizzards[3])
        print()

        if (pos, map(lambda b: frozenset(b), blizzards)) in seen:
            continue
        else:
            seen.add((pos, map(lambda b: frozenset(b), blizzards)))

        if pos == (COLS - 1, ROWS - 1):
            print("Done at minute:", minutes + 1)
            break

        # Step all the blizzards
        blizzards = step_all(blizzards[0], blizzards[1], blizzards[2], blizzards[3])

        # Generate all the next positions based on the current pos
        next_positions = get_next_positions(pos)

        # Check which of the next positions are valid next positions
        # and add them to a queue
        for next_pos in next_positions:
            if (
                next_pos in blizzards[0]
                or next_pos in blizzards[1]
                or next_pos in blizzards[2]
                or next_pos in blizzards[3]
            ):
                pass
            else:
                candidate_state.append((next_pos, minutes + 1, blizzards))

    # print()
    # print("Round 1")
    # print_map(northers, easters, southers, westers)

    # northers, easters, southers, westers = step_all(
    #     northers, easters, southers, westers
    # )

    # print()
    # print("Round 2")
    # print_map(northers, easters, southers, westers)

    print("Done")
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
