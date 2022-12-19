# from functools import reduce, cache
import functools
import operator
import collections
from itertools import chain
import re

from tqdm import tqdm


def workers_str(w):
    return f"o:{w[0]} c:{w[1]} b:{w[2]} g:{w[3]}"


def get_input():
    r = open("full_input.txt")

    return [list(map(int, re.findall(r"\d+", l)[1:])) for l in r]


@functools.lru_cache(maxsize=None)
def maximum_geodes(
    recipe,
    ore,
    clay,
    obsidian,
    ore_robots,
    clay_robots,
    obsidian_robots,
    geode_robots,
    t,
):
    # Do different moves based on the current state
    # Return the max of each of those branches
    # print(
    #     f"Minute: {25-t} resources (ore{ore} clay{clay} obs{obsidian}) workers (ow{ore_robots} cw{clay_robots} bw{obsidian_robots} gw{geode_robots})"
    # )

    if t == 0:
        # print(f"ore{ore} clay{clay} obs{obsidian}")
        return 0

    options = []

    if ore >= recipe[4] and obsidian >= recipe[5]:
        # print("create geo robot")
        options.append(
            maximum_geodes(
                recipe,
                ore - recipe[4] + ore_robots,
                clay + clay_robots,
                obsidian - recipe[5] + obsidian_robots,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
                t - 1,
            )
        )

        # return geode_robots + max(options)

    if ore >= recipe[2] and clay >= recipe[3] and obsidian_robots <= recipe[5]:
        # print("create obs robot")
        options.append(
            maximum_geodes(
                recipe,
                ore - recipe[2] + ore_robots,
                clay - recipe[3] + clay_robots,
                obsidian + obsidian_robots,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
                t - 1,
            )
        )

    if ore >= recipe[1] and clay_robots <= recipe[3]:
        # print("create cla robot")
        options.append(
            maximum_geodes(
                recipe,
                ore - recipe[1] + ore_robots,
                clay + clay_robots,
                obsidian + obsidian_robots,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
                t - 1,
            )
        )

    if ore >= recipe[0] and ore_robots <= max(
        recipe[0], recipe[1], recipe[2], recipe[4]
    ):
        # print("create ore robot")
        options.append(
            maximum_geodes(
                recipe,
                ore - recipe[0] + ore_robots,
                clay + clay_robots,
                obsidian + obsidian_robots,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
                t - 1,
            )
        )

    # Only do nothing if we haven't been able to build a robot this turn
    options.append(
        maximum_geodes(
            recipe,
            ore + ore_robots,
            clay + clay_robots,
            obsidian + obsidian_robots,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            t - 1,
        )
    )

    return geode_robots + max(options)


def calculate_part1(puzzle_input):
    # print("Calculating for", puzzle_input)

    # return maximum_geodes(tuple([2, 3, 3, 8, 3, 12]), 0, 0, 0, 1, 0, 0, 0, 24)
    # return maximum_geodes(tuple([4, 2, 3, 14, 2, 7]), 0, 0, 0, 1, 0, 0, 0, 24)

    quality_level_sum = 0

    for i in tqdm(range(len(puzzle_input))):
        blueprint = puzzle_input[i]
        ql = (i + 1) * maximum_geodes(tuple(blueprint), 0, 0, 0, 1, 0, 0, 0, 24)

        quality_level_sum += ql

    return quality_level_sum

    # # m = maximum_geodes(tuple([4, 2, 3, 14, 2, 7]), 0, 0, 0, 1, 0, 0, 0, 24)
    # m = maximum_geodes(tuple([2, 3, 3, 8, 3, 12]), 0, 0, 0, 1, 0, 0, 0, 24)


def calculate_part2(puzzle_input):
    r = []

    for i in tqdm(range(3)):
        r.append(maximum_geodes(tuple(puzzle_input[i]), 0, 0, 0, 1, 0, 0, 0, 32))

    return r[0] * r[1] * r[2]


def part1(puzzle_input):
    return calculate_part1(puzzle_input)


def part2(puzzle_input):
    return calculate_part2(puzzle_input)


def test_calculation():
    assert maximum_geodes(tuple([4, 2, 3, 14, 2, 7]), 0, 0, 0, 1, 0, 0, 0, 24) == 9
    assert maximum_geodes(tuple([2, 3, 3, 8, 3, 12]), 0, 0, 0, 1, 0, 0, 0, 24) == 12

    # assert maximum_geodes(tuple([1, 2, 3, 4, 5, 6]), 1, 1, 1, 1, 2, 2, 2, 2, 1) == 3
    # assert maximum_geodes(tuple([1, 2, 3, 4, 5, 6]), 1, 1, 1, 1, 5, 2, 6, 2, 2) == 5


if __name__ == "__main__":
    puzzle_input = get_input()

    # print("Part 1 answer:", part1(puzzle_input))
    print("Part 2 answer:", part2(puzzle_input))
