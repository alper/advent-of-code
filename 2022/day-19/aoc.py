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


def maximum_geodes(
    recipe,
    t,
):
    # Final rewrite for part2 strongly inspired by:
    # https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/19.py
    best = 0
    # Current state
    state = (0, 0, 0, 0, 1, 0, 0, 0, t)

    queue = collections.deque([state])
    seen = set()

    while queue:
        state = queue.popleft()
        # print(state)

        (
            ore,
            clay,
            obsidian,
            geodes,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            t,
        ) = state

        best = max(best, geodes)

        if t == 0:
            continue

        # Prune useless paths
        max_ore_needed = max(recipe[0], recipe[1], recipe[2], recipe[4])
        if ore_robots >= max_ore_needed:
            ore_robots = max_ore_needed
        if clay_robots >= recipe[3]:
            clay_robots = recipe[3]
        if obsidian_robots >= recipe[5]:
            obsidian_robots = recipe[5]
        if ore >= t * max_ore_needed - ore_robots * (t - 1):
            ore = t * max_ore_needed - ore_robots * (t - 1)
        if clay >= t * recipe[3] - clay_robots * (t - 1):
            clay = t * recipe[3] - clay_robots * (t - 1)
        if obsidian >= t * recipe[5] - obsidian_robots * (t - 1):
            obsidian = t * recipe[5] - clay_robots * (t - 1)

        # Hopeless situations where we can't win anymore
        if geodes + (t * geode_robots) + sum(range(t, 0, -1)) < best:
            seen.add(state)
            continue

        if state in seen:
            continue
        seen.add(state)

        if len(seen) % 100000 == 0:
            print(t, best, len(seen))

        # Buy no robots
        queue.append(
            (
                ore + ore_robots,
                clay + clay_robots,
                obsidian + obsidian_robots,
                geodes + geode_robots,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots,
                t - 1,
            )
        )

        if ore >= recipe[0]:
            queue.append(
                (
                    ore + ore_robots - recipe[0],
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geodes + geode_robots,
                    ore_robots + 1,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
                    t - 1,
                )
            )

        if ore >= recipe[1]:
            queue.append(
                (
                    ore + ore_robots - recipe[1],
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geodes + geode_robots,
                    ore_robots,
                    clay_robots + 1,
                    obsidian_robots,
                    geode_robots,
                    t - 1,
                )
            )

        if ore >= recipe[2] and clay >= recipe[3]:
            queue.append(
                (
                    ore + ore_robots - recipe[2],
                    clay + clay_robots - recipe[3],
                    obsidian + obsidian_robots,
                    geodes + geode_robots,
                    ore_robots,
                    clay_robots,
                    obsidian_robots + 1,
                    geode_robots,
                    t - 1,
                )
            )

        if ore >= recipe[4] and obsidian >= recipe[5]:
            queue.append(
                (
                    ore + ore_robots - recipe[4],
                    clay + clay_robots,
                    obsidian + obsidian_robots - recipe[5],
                    geodes + geode_robots,
                    ore_robots,
                    clay_robots,
                    obsidian_robots,
                    geode_robots + 1,
                    t - 1,
                )
            )
    return best


def calculate_part1(puzzle_input):
    # print("Calculating for", puzzle_input)

    # return maximum_geodes(tuple([2, 3, 3, 8, 3, 12]), 24)
    # return maximum_geodes(tuple([4, 2, 3, 14, 2, 7]), 24)

    quality_level_sum = 0

    for i in tqdm(range(len(puzzle_input))):
        blueprint = puzzle_input[i]
        print("Blueprint:", blueprint)

        geodes = maximum_geodes(tuple(blueprint), 24)
        print("Geodes:", geodes)

        ql = (i + 1) * geodes

        quality_level_sum += ql

    return quality_level_sum


def calculate_part2(puzzle_input):
    r = []

    for i in tqdm(range(3)):
        r.append(maximum_geodes(tuple(puzzle_input[i]), 32))
        print(i, r)

    return r[0] * r[1] * r[2]


def part1(puzzle_input):
    return calculate_part1(puzzle_input)


def part2(puzzle_input):
    return calculate_part2(puzzle_input)


def test_calculation():
    assert maximum_geodes(tuple([4, 2, 3, 14, 2, 7]), 24) == 9
    assert maximum_geodes(tuple([2, 3, 3, 8, 3, 12]), 24) == 12

    # First full run
    assert maximum_geodes(tuple([3, 4, 2, 20, 4, 7]), 24) == 2
    assert maximum_geodes(tuple([3, 4, 3, 19, 3, 8]), 24) == 1
    assert maximum_geodes(tuple([4, 4, 2, 14, 4, 15]), 24) == 0
    assert maximum_geodes(tuple([4, 2, 2, 16, 2, 8]), 24) == 8
    assert maximum_geodes(tuple([4, 4, 3, 14, 4, 8]), 24) == 1
    assert maximum_geodes(tuple([4, 3, 2, 7, 3, 8]), 24) == 11
    assert maximum_geodes(tuple([2, 3, 2, 16, 2, 9]), 24) == 7
    assert maximum_geodes(tuple([3, 3, 3, 9, 2, 10]), 24) == 7
    assert maximum_geodes(tuple([2, 4, 3, 14, 4, 9]), 24) == 6
    assert maximum_geodes(tuple([4, 4, 2, 7, 3, 10]), 24) == 5
    assert maximum_geodes(tuple([2, 4, 4, 13, 3, 11]), 24) == 5


if __name__ == "__main__":
    puzzle_input = get_input()

    # print("Part 1 answer:", part1(puzzle_input))
    print("Part 2 answer:", part2(puzzle_input))
