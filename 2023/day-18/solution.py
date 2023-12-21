import sys

sample_data = """R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"""

real_data = sample_data
real_data = open('input.txt').read()

plan = []
for step in real_data.split('\n'):
    direction, distance, color = step.split()
    plan.append((direction, int(distance), color))

origin = (1000, 1000)

min_x = 1000
max_x = 1000
min_y = 1000
max_y = 1000

pos = origin
ground = {}

ground[pos] = '#'

area = 0
perimeter = 1

for step in plan:
    direction, distance, color = step

    for i in range(1, distance+1):
        perimeter += 1

        if direction == 'R':
            x, y = (pos[0] + i, pos[1])
        elif direction == 'L':
            x, y = (pos[0] - i, pos[1])
        elif direction == 'U':
            x, y = (pos[0], pos[1] + i)
        elif direction == 'D':
            x, y = (pos[0], pos[1] - i)

        ground[(x, y)] = '#'

        min_x = min(min_x, x)
        max_x = max(max_x, x)
        min_y = min(min_y, y)
        max_y = max(max_y, y)

    area += (pos[0] * y) - (pos[1] * x)

    pos = (x, y)

area += (pos[0] * origin[1]) - (pos[1] * origin[0])
area = abs(area)
area /= 2

area = area + (perimeter / 2) + 1

print("Area calculated:", area)

print("Lava after dig", len(ground))

print()

# print(ground.get((999, 1000)))
# print(ground.get((1000, 1001)))
# print(ground.get((1000, 1000)))
# print(ground.get((1001, 1000)))

s1 = []
for y in range(max_y + 10, min_y - 10, -1):
    s2 = []
    for x in range(min_x-10, max_x + 10):
        # print(ground.get((1001, 1000)))
        s = ground.get((x, y), '.')
        # print(x, y, s)
        s2.append(s)
    s1.append(''.join(s2))
print ('\n'.join(s1))


# Part 2

origin = (0, 0)

pos = origin

area = 0
perimeter = 1

for step in plan:
    direction, distance, color = step
    print(step)
    print(color[2: -2])
    distance = int(color[2: -2], 16)
    direction = {'0': 'R', '1': 'D', '2': 'L', '3': 'U'}[color[-2]]

    print(color, direction, distance)

    perimeter += distance

    if direction == 'R':
        x, y = (pos[0] + distance, pos[1])
    elif direction == 'L':
        x, y = (pos[0] - distance, pos[1])
    elif direction == 'U':
        x, y = (pos[0], pos[1] + distance)
    elif direction == 'D':
        x, y = (pos[0], pos[1] - distance)

    area += (pos[0] * y) - (pos[1] * x)

    pos = (x, y)

area += (pos[0] * origin[1]) - (pos[1] * origin[0])
area = abs(area)
area /= 2

area = area + (perimeter / 2) + 1

print("Area part 2", area)