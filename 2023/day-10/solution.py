from typing import Dict, Tuple, List
import math

sample_input = """-L|F7
7S-7|
L|7||
-L-J|
L|-JF"""

sample_input2 = """7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"""

real_input = sample_input
real_input = open("input.txt").read()

locations = {}

lines = real_input.splitlines()

start = None
for y, line in enumerate(lines):
    for x, char in enumerate(line):
        locations[(x, y)] = char

        if char == 'S':
            start = (x, y)

print(locations)

print("Start:", start)
locs = [start]

def add_location(locs: List[Tuple[int, int]], locations: Dict[Tuple[int, int], str]) -> (int, int):
    x, y = locs[-1]
    pipe = locations.get(locs[-1])

    if not pipe:
        return locs

    # if pipe == 'S':
    #     pipe = 'F'

    if pipe == '|':
        if locs[-2][1] == y-1:
            n = (x, y+1)
            if locations.get(n) in ['|', 'L', 'J']:
                locs.append(n)
        else:
            n = (x, y-1)
            if locations.get(n) in ['|', '7', 'F']:
                locs.append(n)
    elif pipe == '-':
        if locs[-2][0] == x-1:
            n = (x+1, y)
            if locations.get(n) in ['-', 'J', '7']:
                locs.append(n)
        else:
            n = (x-1, y)
            if locations.get(n) in ['-', 'L', 'F']:
                locs.append(n)
    elif pipe == 'L':
        if locs[-2][1] == y-1:
            n = (x+1, y)
            if locations.get(n) in ['-', 'J', '7']:
                locs.append(n)
        else:
            n = (x, y-1)
            if locations.get(n) in ['|', '7', 'F']:
                locs.append(n)
    elif pipe == 'J':
        if locs[-2][1] == y-1:
            n = (x-1, y)
            if locations.get(n) in ['-', 'L', 'F']:
                locs.append(n)
        else:
            n = (x, y-1)
            if locations.get(n) in ['|', '7', 'F']:
                locs.append(n)
    elif pipe == '7':
        if locs[-2][0] == x-1:
            n = (x, y+1)
            if locations.get(n) in ['|', 'L', 'J']:
                locs.append(n)
        else:
            n = (x-1, y)
            if locations.get(n) in ['-', 'L', 'F']:
                locs.append(n)
    elif pipe == 'F':
        if locs[-2][0] == x+1:
            n = (x, y+1)
            if locations.get(n) in ['|', 'L', 'J']:
                locs.append(n)
        else:
            n = (x+1, y)
            if locations.get(n) in ['-', 'J', '7']:
                locs.append(n)

    return locs


counter = 0
locs = [start, (1, 2)]

pipe_types = ['|', '-', 'L', 'J', '7', 'F']

candidates = [
    [start, (start[0], start[1]-1)], #|
    [start, (start[0], start[1]+1)], #|
    [start, (start[0]+1, start[1])], #-
    [start, (start[0]-1, start[1])], #-
    [start, (start[0], start[1]-1)], #L
    [start, (start[0]+1, start[1])], #L
    [start, (start[0]-1, start[1])], #J
    [start, (start[0], start[1]-1)], #J
    [start, (start[0]-1, start[1])], #7
    [start, (start[0], start[1]+1)], #7
    [start, (start[0]+1, start[1])], #F
    [start, (start[0], start[1]+1)], #F
]

lengths = []

for candidate in candidates:
    print("Trying:", candidate)

    while True:
        prev_l = len(candidate)

        candidate = add_location(candidate, locations)

        if len(candidate) == prev_l:
            break

        if candidate[-1] == candidate[0]:
            break

    print("Candidate:", candidate)
    print("Pipes:", [locations.get(c) for c in candidate])
    print("Len locs:", len(candidate))
    print()
    lengths.append(len(candidate))


print("Lengths:", lengths)
print("Max length:", max(lengths))