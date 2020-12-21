import numpy as np


def string_array_to_numpy(list_of_strings):
    return np.array([[c for c in list(s.strip())] for s in list_of_strings])


def tile_header_to_id(header):
    return header.split(" ")[1][:-1]


def arrays_fit_for_any_rotation(arr1, arr2):
    for i in range(4):
        for j in range(4):
            if np.all(np.rot90(arr1, i)[:, -1] == np.rot90(arr2, j)[:, 0]):
                return True

    return False


def arrays_fit_for_rotations_and_mirroring(arr1, arr2):
    return (
        arrays_fit_for_any_rotation(arr1, arr2)
        or arrays_fit_for_any_rotation(arr1, np.fliplr(arr2))
        or arrays_fit_for_any_rotation(arr1, np.flipud(arr2))
    )


t = open("test_input.txt").read()
t = open("input.txt").read()
tiles_list = t.split("\n\n")

tiles = [(tile.split("\n")[0], tile.split("\n")[1:]) for tile in tiles_list]

tiles = {
    tile_header_to_id(tt[0]): string_array_to_numpy(tt[1]) for tt in tiles
}

# print("Testing problematic arrays")
# print(arrays_fit_for_rotations_and_mirroring(tiles["1951"], tiles["2729"]))
# print(np.rot90(tiles["1951"], 3)[:, -1])
# print(np.rot90(tiles["2729"], 3)[:, 0])


neighbors = {}

for tile_id in tiles.keys():
    print(f"Testing {tile_id}")
    tile = tiles[tile_id]

    # print(f"Testing {tile_id}")

    match_count = 0
    # Go through all the other tiles and see which match
    for other_tile_id, other_tile in tiles.items():
        # print(f"Other {other_tile_id}")

        if tile_id != other_tile_id:
            if arrays_fit_for_rotations_and_mirroring(tile, other_tile):
                match_count += 1
                continue

    neighbors[tile_id] = match_count

print(neighbors)

import math

print([(n, v) for (n, v) in neighbors.items() if v == 2])
print(math.prod([int(n) for (n, v) in neighbors.items() if v == 2]))

corners = ["2749", "2713", "1487", "1063"]

# Pick one corner and then start laying out the image?

im = np.chararray((12 * 10, 12 * 10))
im[:] = "."
