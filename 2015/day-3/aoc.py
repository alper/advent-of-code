from functools import reduce
import operator


def get_input():
    return open("full_input.txt").read()


def calculate_part1(i):
    print("Calculating for", i)
    houses = set()

    current_position = 0, 0
    houses.add(current_position)

    for move in i:
        if move == ">":
            current_position = current_position[0] + 1, current_position[1]
        elif move == "v":
            current_position = current_position[0], current_position[1] - 1
        elif move == "<":
            current_position = current_position[0] - 1, current_position[1]
        elif move == "^":
            current_position = current_position[0], current_position[1] + 1

        print(current_position)
        houses.add(current_position)

    return len(houses)


def calculate_part2(i):
    print("Calculating", i)

    houses = set()

    santa_position = 0, 0
    robo_position = 0, 0
    houses.add(santa_position)
    houses.add(robo_position)
    print(houses)

    for m1, m2 in (i[pos : pos + 2] for pos in range(0, len(i), 2)):
        print(m1, m2)

        def next_position(position, move):
            if move == ">":
                return position[0] + 1, position[1]
            elif move == "v":
                return position[0], position[1] - 1
            elif move == "<":
                return position[0] - 1, position[1]
            elif move == "^":
                return position[0], position[1] + 1

        santa_position = next_position(santa_position, m1)
        robo_position = next_position(robo_position, m2)

        print("Santa and robot", santa_position, robo_position)

        houses.add(santa_position)
        houses.add(robo_position)

    return len(houses)


def part1(i):
    return calculate_part1(i)


def part2(i):
    return reduce(operator.__add__, map(calculate_part2, i.split("\n")), 0)


def test_calculation():
    assert calculate_part1(">") == 2
    assert calculate_part1("^>v<") == 4
    assert calculate_part1("^v^v^v^v^v") == 2

    assert calculate_part2("^v") == 3
    assert calculate_part2("^>v<") == 3
    assert calculate_part2("^v^v^v^v^v") == 11


if __name__ == "__main__":
    i = get_input()

    print("Part 1 answer:", part1(i))
    print("Part 2 answer:", part2(i))
