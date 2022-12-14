from functools import reduce
import operator
import collections
from itertools import chain
from hashlib import md5


def get_input():
    return "ckczppom"


def calculate(i, zeroes):
    print("Calculating for", i)

    counter = 0

    while True:
        if md5(bytes(i + str(counter), "utf-8")).hexdigest()[:zeroes] == ("0" * zeroes):
            return counter

        counter += 1

        if counter % 1000000 == 0:
            print("Countuer: ", counter)


def part1(i):
    return calculate(i, 5)


def part2(i):
    return calculate(i, 6)


def test_calculation():
    assert calculate("abcdef", 5) == 609043
    assert calculate("pqrstuv", 5) == 1048970


if __name__ == "__main__":
    i = get_input()

    print("Part 1 answer:", part1(i))
    print("Part 2 answer:", part2(i))
