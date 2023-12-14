sample_input = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""

real_input = sample_input
real_input = open('input.txt', 'r').read()

grid = {}

for i, row in enumerate(real_input.splitlines()):
    for j, c in enumerate(row):
        print("J", j, c)

        grid[(i, j)] = c

max_row = max([c[0] for c in grid.keys()])
max_col = max([c[1] for c in grid.keys()])

print("Grid as we got it")
for i in range(max_row+1):
    for j in range(max_col+1):
        print(grid.get((i, j)), end="")
    print()

print()
print()
print(max_row, max_col)
print()
print()

for i in range(max_row+1):
    for j in range(max_col+1):
        if grid.get((i, j)) == 'O':
            print()
            print("Found O at", i, j)
            # See if we can move it up
            for k in range(i-1, -1, -1):
                if grid.get((k, j)) == '.':
                    print("Swapping", i, k, j)
                    # Swap
                    grid[(k, j)] = 'O'
                    grid[(k+1, j)] = '.'
                elif grid.get((k, j)) == '#' or grid.get((k, j)) == 'O':
                    break

print("Grid after rolling everything to the north")
for i in range(max_row+1):
    for j in range(max_col+1):
        print(grid.get((i, j)), end="")
    print()

print("Calculate load")

pressure = 0
for i in range(max_row+1):
    for j in range(max_row+1):
        if grid.get((i, j)) == 'O':
            l = max_row + 1 - i

            pressure += l

print("Total pressure =", pressure)
