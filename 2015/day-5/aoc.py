from functools import reduce
import operator
import collections
from itertools import chain


def chunker(seq, size):
    return (seq[pos : pos + size] for pos in range(0, len(seq), size))


def get_input():
    return open("full_input.txt").read()


def calculate_part1(i):
    print("Calculating: ", i)

    vowel_criterion = (
        len(
            list(
                filter(
                    lambda x: "a" in x or "e" in x or "i" in x or "o" in x or "u" in x,
                    i,
                )
            )
        )
        >= 3
    )

    double_criterion = False
    for index in range(len(i) - 1):
        w = i[index : index + 2]

        if w[0] == w[1]:
            double_criterion = True

    exclusion_criterion = (
        "ab" not in i and "cd" not in i and "pq" not in i and "xy" not in i
    )

    print("Checks: ", vowel_criterion, double_criterion, exclusion_criterion)

    return vowel_criterion and double_criterion and exclusion_criterion


def calculate_part2(i):
    print("Calculating: ", i)

    double_criterion = False
    for c in range(len(i) - 1):
        two_sub = i[c : c + 2]

        print("Trying", two_sub)

        first_index = i.find(two_sub)
        second_index = i.find(two_sub, first_index + 2)

        print("Indexes:", first_index, second_index)

        if first_index != -1 and second_index != -1:
            double_criterion = True
            break

    skip_criterion = False
    for index in range(len(i) - 2):
        w = i[index : index + 3]

        if w[0] == w[2]:
            skip_criterion = True

    print("Checks: ", double_criterion, skip_criterion)

    return double_criterion and skip_criterion


def part1(i):
    return len(list(filter(lambda x: x, map(calculate_part1, i.split("\n")))))


def part2(i):
    return len(list(filter(lambda x: x, map(calculate_part2, i.split("\n")))))


def test_calculation():
    assert calculate_part1("ugknbfddgicrmopn") == True
    assert calculate_part1("aaa") == True
    assert calculate_part1("jchzalrnumimnmhp") == False
    assert calculate_part1("haegwjzuvuyypxyu") == False
    assert calculate_part1("dvszwmarrgswjxmb") == False

    assert calculate_part2("qjhvhtzxzqqjkmpb") == True
    assert calculate_part2("xxyxx") == True
    assert calculate_part2("uurcxstgmygtbstg") == False
    assert calculate_part2("ieodomkazucvgmuy") == False


if __name__ == "__main__":
    i = get_input()

    print("Part 1 answer:", part1(i))
    print("Part 2 answer:", part2(i))
