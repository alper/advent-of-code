from typing import Dict, TypeAlias, Tuple

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

class Grid:
    data: Dict[Tuple[int, int], str]

    width: int
    height: int

    def __init__(self, data):
        self.data = data

        self.height = max([c[0] for c in self.data.keys()]) + 1
        self.width = max([c[1] for c in self.data.keys()]) + 1


    @classmethod
    def parse(self, raw_string):
        grid = {}
    
        for i, row in enumerate(raw_string.splitlines()):
            for j, c in enumerate(row):
                # print("J", j, c)

                grid[(i, j)] = c

        return Grid(grid)

    def tilt_north(self):
        for i in range(self.height):
            for j in range(self.width):
                if self.data.get((i, j)) == 'O':
                    # See if we can move it up
                    for k in range(i-1, -1, -1):
                        if self.data.get((k, j)) == '.':
                            # Swap
                            self.data[(k, j)] = 'O'
                            self.data[(k+1, j)] = '.'
                        elif self.data.get((k, j)) == '#' or self.data.get((k, j)) == 'O':
                            break

    def tilt_south(self):
        for i in range(self.height-1, -1, -1):
            for j in range(self.width):
                if self.data.get((i, j)) == 'O':
                    # See if we can move it down
                    for k in range(i+1, self.height):
                        if self.data.get((k, j)) == '.':
                            self.data[(k, j)] = 'O'
                            self.data[(k-1, j)] = '.'
                        elif self.data.get((k, j)) == '#' or self.data.get((k, j)) == 'O':
                            break

    def tilt_east(self):
        for j in range(self.width-1, -1, -1):
            for i in range(self.height):
                if self.data.get((i, j)) == 'O':
                    # Move it east
                    for k in range(j+1, self.width):
                        if self.data.get((i, k)) == '.':
                            self.data[(i, k)] = 'O'
                            self.data[(i, k-1)] = '.'
                        elif self.data.get((i, k)) == '#' or self.data.get((i, k)) == 'O':
                            break

    def tilt_west(self):
        for j in range(self.width):
            for i in range(self.height):
                if self.data.get((i, j)) == 'O':
                    for k in range(j-1, -1, -1):
                        if self.data.get((i, k)) == '.':
                            self.data[(i, k)] = 'O'
                            self.data[(i, k+1)] = '.'
                        elif self.data.get((i, k)) == '#' or self.data.get((i, k)) == 'O':
                            break

    def cycle(self):
        self.tilt_north()
        self.tilt_west()
        self.tilt_south()
        self.tilt_east()
        

    def calculate_load(self) -> int:
        pressure = 0

        for i in range(self.height):
            for j in range(self.width):
                if self.data.get((i, j)) == 'O':
                    l = self.height - i

                    pressure += l
        return pressure

    def get_key(self) -> int:
        return hash(str(self))

    def __str__(self):
        grid_components = []
        for i in range(self.height):
            row_components = []
            for j in range(self.width):
                row_components.append(self.data.get((i, j)))

            grid_components.append(''.join(row_components))
        return '\n'.join(grid_components)
       

seen = {}

grid = Grid.parse(real_input)

print("Part 1")
grid.tilt_north()
print(grid)
print("Load:", grid.calculate_load())
print()


i = 0
steps = 1000000000 

while i < steps:
    print("i", i)
    key = grid.get_key()
    print(key)
    print("Load:", grid.calculate_load())
    print(grid)
    print()

    if key in seen:
        print(f"Found a match: step {i} == step {seen[key]}")
        
        loop_size = i - seen[key]
        remains = (steps - i) % loop_size
        i = steps - remains

    grid.cycle()


    seen[key] = i

    if i % 1000 == 0:
        print("At step:", i)

    i += 1



print("Solution: ", grid.calculate_load())
