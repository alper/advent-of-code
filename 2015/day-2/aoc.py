from functools import reduce
import operator


def get_input():
    return open("full_input.txt").read()


def calculate_part1(i):
    l, w, h = map(int, i.split("x"))

    return (2 * l * w) + (2 * w * h) + (2 * h * l) + min((l * w), (w * h), (h * l))


def calculate_part2(i):
    l, w, h = map(int, i.split("x"))

    return sorted([l, w, h])[0] * 2 + sorted([l, w, h])[1] * 2 + (l * w * h)


def part1(i):
    return reduce(operator.__add__, map(calculate_part1, i.split("\n")), 0)


def part2(i):
    return reduce(operator.__add__, map(calculate_part2, i.split("\n")), 0)


def test_calculation():
    assert calculate_part1("2x3x4") == 58
    assert calculate_part1("1x1x10") == 43

    assert calculate_part2("2x3x4") == 34
    assert calculate_part2("1x1x10") == 14


if __name__ == "__main__":
    i = get_input()

    print(part1(i))
    print(part2(i))
