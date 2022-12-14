from functools import reduce
import operator
from collections import defaultdict
from itertools import chain
import pyparsing as pp


def get_input():
    coord = pp.Word(pp.nums) + pp.Word(",") + pp.Word(pp.nums)
    instruction = pp.Word("toggle") ^ pp.Word("turn on") ^ pp.Word("turn off")

    parser = instruction + coord + pp.Word("through") + coord
    lines = open("full_input.txt").readlines()

    p = []
    for l in lines:
        parsed = parser.parse_string(l)
        p.append(
            (
                parsed[0].strip(),
                int(parsed[1]),
                int(parsed[3]),
                int(parsed[5]),
                int(parsed[7]),
            )
        )

    return p


def calculate_part1(puzzle_input):
    print("Calculating for", i)

    grid = defaultdict(lambda: False)

    for instruction in puzzle_input:
        if instruction[1] > instruction[3] or instruction[2] > instruction[4]:
            print("Error invariant not met", instruction)

        for a in range(instruction[1], instruction[3] + 1):
            for b in range(instruction[2], instruction[4] + 1):
                current = grid[(a, b)]

                if instruction[0] == "turn on":
                    grid[(a, b)] = True
                elif instruction[0] == "turn off":
                    grid[(a, b)] = False
                elif instruction[0] == "toggle":
                    grid[(a, b)] = not current
    return grid


def calculate_part2(puzzle_input):
    print("Calculating for", i)

    grid = defaultdict(lambda: 0)

    for instruction in puzzle_input:
        if instruction[1] > instruction[3] or instruction[2] > instruction[4]:
            print("Error invariant not met", instruction)

        for a in range(instruction[1], instruction[3] + 1):
            for b in range(instruction[2], instruction[4] + 1):
                if instruction[0] == "turn on":
                    grid[(a, b)] += 1
                elif instruction[0] == "turn off":
                    if grid[(a, b)] > 0:
                        grid[(a, b)] -= 1
                elif instruction[0] == "toggle":
                    grid[(a, b)] += 2
    return grid


def part1(i):
    g = calculate_part1(i)

    return len(list(filter(lambda x: x, g.values())))


def part2(i):
    g = calculate_part2(i)

    return sum(g.values())


if __name__ == "__main__":
    i = get_input()
    print(i)

    # print("Part 1 answer:", part1(i))
    print("Part 2 answer:", part2(i))
