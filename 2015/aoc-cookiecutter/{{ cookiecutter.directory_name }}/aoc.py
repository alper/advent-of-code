from functools import reduce
import operator
import collections
from itertools import chain


def get_input():
    return open("full_input.txt").read()


def calculate_part1(i):
    print("Calculating for", i)

    return 1


def calculate_part2(i):
    print("Calculating", i)

    return 1


def part1(i):
    return calculate_part1(i)


def part2(i):
    return calculate_part2(i)


def test_calculation():
    assert calculate_part1(">") == false

    # assert calculate_part2("^v") == false


if __name__ == "__main__":
    i = get_input()

    # print("Part 1 answer:", part1(i))
    # print("Part 2 answer:", part2(i))
