import collections

sample_input = """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"""

real_input = sample_input
real_input = open("input.txt").read()

lines = real_input.split("\n")

instructions_parsed = collections.deque(list(lines[0].strip()))

branches = {line[0:3]: (line[7:10], line[12:15]) for line in lines[2:]}

position = 'AAA'
instructions = instructions_parsed.copy()
steps = 0

while True:
    move = instructions.popleft()
    steps += 1

    if move == 'L':
        position = branches[position][0]
    else:
        position = branches[position][1]

    instructions.append(move)

    if position == "ZZZ":
        break

print("Solutino 1", steps)

positions = [key for key in branches.keys() if key[2] == 'A']
until_end = {}

for position in positions:
    print("Position", position)

    steps = 0
    instructions = instructions_parsed.copy()
    location = position

    while True:
        steps += 1
        move = instructions.popleft()

        location = branches[location][0 if move == 'L' else 1]

        if location[2] == 'Z':
            break

        instructions.append(move)

    until_end[position] = steps

print(until_end)

import math

print("Solution 2", math.lcm(*until_end.values()))