import re

sample_input = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""

real_input = sample_input
real_input = open('input.txt', 'r').read()

lines = real_input.split('\n')

pattern_num = re.compile('\d+')

s = 0

hit_list_per_part = {}

for i, line in enumerate(lines):
    print("line", line)

    for match in re.finditer(pattern_num, line):
        print("match", match.group(), match.span())

        part_number = int(match.group())

        for j in range(match.span()[0], match.span()[1]):
            hit_list_per_part[(i, j)] = part_number

        # Check validity
        valid = False

        # Current line before
        if match.span()[0] > 0:
            if line[match.span()[0]-1] != '.':
                valid = True

        # Current line after
        if match.span()[1] < len(line):
            if line[match.span()[1]] != '.':
                valid = True

        # Previous line
        if i > 0:
            sel = lines[i-1][max(0, match.span()[0]-1):match.span()[1]+1]
            print("Selection previous: ", sel)

            if len(list(filter(lambda x: x != '.', sel))) > 0:
                valid = True

        if i < len(lines)-1:
            sel = lines[i+1][max(0, match.span()[0]-1):match.span()[1]+1]
            print("Selection next: ", sel)

            if len(list(filter(lambda x: x != '.', sel))) > 0:
                valid = True

        # Next line

        if not valid:
            print("Not valid")
        print()

        if valid:
            s += part_number

print("Solution part 1", s)
print()
print()

ratios_sum = 0

for key, value in hit_list_per_part.items():
    if value == 491:
        print(key)

for i, line in enumerate(lines):
    for j, c in enumerate(line):
        if c == '*':
            parts = [
                hit_list_per_part.get((i-1, j-1), None),
                hit_list_per_part.get((i-1, j), None),
                hit_list_per_part.get((i-1, j+1), None),
                hit_list_per_part.get((i, j-1), None),
                hit_list_per_part.get((i, j+1), None),
                hit_list_per_part.get((i+1, j-1), None),
                hit_list_per_part.get((i+1, j), None),
                hit_list_per_part.get((i+1, j+1), None)
            ]

            parts = list(set(filter(lambda x: x != None, parts)))

            if len(parts) == 2:
                ratios_sum += parts[0] * parts[1]

            print("For ", i, " ", j, " ", parts)

print("Solution part 2", ratios_sum)