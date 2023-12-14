import collections
from typing import List

from itertools import islice

def window(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result

sample1 = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."""

sample2 = """#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""

real_input = open('input.txt', 'r').read().split('\n\n')

def parse_grid(sample: str) -> List[List[str]]:
    return [list(l) for l in sample.strip().splitlines()]

def format_grid(grid: List[List[str]]) -> str:
    return '\n'.join([''.join(l) for l in grid])


def row_symmetry(grid: List[List[str]]) -> int:
    print("Checking row symmetry")
    for i, j in window(range(len(grid)), 2):
        if grid[i] == grid[j]:
            # This is a candidate for symmetry
            symmetry_point = i
            print("Candidate:", symmetry_point)

            rows_to_check = min(i, len(grid)-j-1)

            failed = False
            for k in range(1, rows_to_check+1):
                if grid[i-k] != grid[j+k]:
                    failed = True
                    break

            if not failed:
                return symmetry_point+1
    return -1

def column_symmetry(grid: List[List[str]]) -> int:
    print("Checking column symmetry")
    transposed = list(map(list, zip(*grid)))

    return row_symmetry(transposed)

def row_diff(r1: List[str], r2: List[str]) -> int:
    d = 0
    for a, b in zip(r1, r2):
        if a != b:
            d += 1
    return d

def row_near_symmetry(grid: List[List[str]]) -> int:
    print("Checking row near symmetry")
    for i, j in window(range(len(grid)), 2):
        if row_diff(grid[i], grid[j]) <= 1:
            # This is a candidate for symmetry
            symmetry_point = i
            print("Candidate:", symmetry_point)

            rows_to_check = min(i, len(grid)-j-1)

            diff_count = row_diff(grid[i], grid[j])

            for k in range(1, rows_to_check+1):
                diff_count += row_diff(grid[i-k], grid[j+k])

                if diff_count > 1:
                    break

            if diff_count == 1:
                return symmetry_point+1
    return -1

def column_near_symmetry(grid: List[List[str]]) -> int:
    print("Checking column symmetry")
    transposed = list(map(list, zip(*grid)))

    return row_near_symmetry(transposed)

# print("Col")
# print(column_symmetry(parse_grid(sample1)))

# print("Row")
# print(row_symmetry(parse_grid(sample2)))

print("Real input")

s = 0

for g in real_input:
    print("Doing grid")
    grid = parse_grid(g)

    print(format_grid(grid))

    r = row_near_symmetry(grid)
    c = column_near_symmetry(grid)

    if c != -1:
        print(f"Column symmetry at position {c}")
        s += c
    elif r != -1:
        print(f"Row symmetry at position {r}")
        s += (100*r)

    print()
    print()

print("Solution 1", s)

