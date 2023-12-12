import itertools
from typing import Tuple, Set

sample_input = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""

real_input = sample_input
real_input = open("input.txt").read()

parsed = [list(l) for l in real_input.split("\n")]

galaxy_rows = set()
galaxy_cols = set()

galaxies = []

for i in range(len(parsed)):
    for j in range(len(parsed[i])):
        if parsed[i][j] == "#":
            galaxy_rows.add(i)
            galaxy_cols.add(j)

            # x,y
            galaxies.append((j, i))

print(galaxies)

def calculate_distance(g1: Tuple[int, int], g2: Tuple[int, int], galaxy_rows: Set[int]=set(), galaxy_cols:Set[int]=set(), ancient=False) -> int:
    x1 = min(g1[0], g2[0])
    x2 = max(g1[0], g2[0])
    y1 = min(g1[1], g2[1])
    y2 = max(g1[1], g2[1])

    dist = 0
    # Calculate distance
    for x in range(x1+1, x2+1):
        # print("X", x)
        dist += 1

        if not x in galaxy_cols:
            if ancient:
                dist += (1000000-1)
            else:
                dist += 1

    for y in range(y1+1, y2+1):
        # print("Y", y)
        dist += 1

        if not y in galaxy_rows:
            if ancient:
                dist += (1000000-1)
            else:
                dist += 1

    return dist

def test_dist():
    assert calculate_distance((3, 0), (7, 8), galaxy_rows, galaxy_cols) == 15
    assert calculate_distance((0, 2), (9, 6), galaxy_rows, galaxy_cols) == 17
    assert calculate_distance((0, 9), (4, 9), galaxy_rows, galaxy_cols) == 5


if __name__ == "__main__":
    total_distance = 0
    for pair in itertools.combinations(galaxies, 2):
        print(pair)

        dist = calculate_distance(pair[0], pair[1], galaxy_rows, galaxy_cols)

        print("Dist", dist)

        total_distance += dist

    print("Solution part 1:", total_distance)

    total_distance = 0
    for pair in itertools.combinations(galaxies, 2):
        print(pair)

        dist = calculate_distance(pair[0], pair[1], galaxy_rows, galaxy_cols, ancient=True)

        print("Dist", dist)

        total_distance += dist

    print("Solution part 2:", total_distance)