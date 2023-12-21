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

real_input = sample_input2
# real_input = open("input.txt").read()

class Field:
    def __init__(self, input: str):
        self.grid: Dict[Tuple[int, int], str] = {}
        self.start: Tuple[int, int] = None

        self.max_y = 0
        self.max_x = 0

        for y, line in enumerate(input.splitlines()):
            for x, char in enumerate(line):
                self.grid[(x, y)] = char

                if char == 'S':
                    self.start = (x, y)

                self.max_x = max(self.max_x, x)
                self.max_y = max(self.max_y, y)

    def str_path(self, path: List[Tuple[int, int]]) -> str:
        out_lines = []

        for i in range(field.max_y+1):
            out_line = []

            for j in range(field.max_x+1):
                if (j, i) in path:
                    out_line.append(field.grid[(j, i)])
                else:
                    out_line.append(' ')
            out_lines.append(''.join(out_line))
        return '\n'.join(out_lines)

    def str_inside_out_path(self, path: List[Tuple[int, int]]) -> str:
        out_lines = []

        for i in range(field.max_y+1):
            out_line = []

            for j in range(field.max_x+1):
                cross_count = 0

                for k in range(max(self.max_y-i, self.max_x-j)):
                    c = self.grid.get((j+k, i+k))

                    pipe_types = ['|', '-', 'L', 'J', '7', 'F']

                    if c in ['|', '-', 'J', 'F']:
                        cross_count += 1

                    inside = False
                    if cross_count % 2 == 0:
                        # Inside
                        inside = True

                    if (j+k, i+k) in path:
                        out_line.append(c)
                    else:
                        if inside:
                            out_line.append('X')
                        else:
                            out_line.append('O')

            out_lines.append(''.join(out_line))
        return '\n'.join(out_lines)

    def __str__(self) -> str:
        out_lines = []

        for i in range(field.max_y+1):
            out_line = []

            for j in range(field.max_x+1):
                out_line.append(field.grid[(j, i)])

            out_lines.append(''.join(out_line))
        return '\n'.join(out_lines)

    def add_to_path(self, path: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
        x, y = path[-1] # Last location
        pipe = self.grid.get(path[-1])

        if not pipe:
            return path

        # if pipe == 'S':
        #     pipe = 'F'

        if pipe == '|':
            if path[-2][1] == y-1:
                n = (x, y+1)
                if self.grid.get(n) in ['|', 'L', 'J']:
                    path.append(n)
            else:
                n = (x, y-1)
                if self.grid.get(n) in ['|', '7', 'F']:
                    path.append(n)
        elif pipe == '-':
            if path[-2][0] == x-1:
                n = (x+1, y)
                if self.grid.get(n) in ['-', 'J', '7']:
                    path.append(n)
            else:
                n = (x-1, y)
                if self.grid.get(n) in ['-', 'L', 'F']:
                    path.append(n)
        elif pipe == 'L':
            if path[-2][1] == y-1:
                n = (x+1, y)
                if self.grid.get(n) in ['-', 'J', '7']:
                    path.append(n)
            else:
                n = (x, y-1)
                if self.grid.get(n) in ['|', '7', 'F']:
                    path.append(n)
        elif pipe == 'J':
            if path[-2][1] == y-1:
                n = (x-1, y)
                if self.grid.get(n) in ['-', 'L', 'F']:
                    path.append(n)
            else:
                n = (x, y-1)
                if self.grid.get(n) in ['|', '7', 'F']:
                    path.append(n)
        elif pipe == '7':
            if path[-2][0] == x-1:
                n = (x, y+1)
                if self.grid.get(n) in ['|', 'L', 'J']:
                    path.append(n)
            else:
                n = (x-1, y)
                if self.grid.get(n) in ['-', 'L', 'F']:
                    path.append(n)
        elif pipe == 'F':
            if path[-2][0] == x+1:
                n = (x, y+1)
                if self.grid.get(n) in ['|', 'L', 'J']:
                    path.append(n)
            else:
                n = (x+1, y)
                if self.grid.get(n) in ['-', 'J', '7']:
                    path.append(n)

        return path

    def get_starting_paths(self) -> List[List[Tuple[int, int]]]:
        return [
            [self.start, (self.start[0], self.start[1]-1)], #|
            [self.start, (self.start[0], self.start[1]+1)], #|
            [self.start, (self.start[0]+1, self.start[1])], #-
            [self.start, (self.start[0]-1, self.start[1])], #-
            [self.start, (self.start[0], self.start[1]-1)], #L
            [self.start, (self.start[0]+1, self.start[1])], #L
            [self.start, (self.start[0]-1, self.start[1])], #J
            [self.start, (self.start[0], self.start[1]-1)], #J
            [self.start, (self.start[0]-1, self.start[1])], #7
            [self.start, (self.start[0], self.start[1]+1)], #7
            [self.start, (self.start[0]+1, self.start[1])], #F
            [self.start, (self.start[0], self.start[1]+1)], #F
        ]

    def get_path_from_start(self) -> List[Tuple[int, int]]:
        candidates = self.get_starting_paths()

        lengths = {}

        for candidate in candidates:
            print("Trying:", candidate)

            while True:
                prev_l = len(candidate)

                candidate = self.add_to_path(candidate)

                if len(candidate) == prev_l:
                    break

                if candidate[-1] == candidate[0]:
                    break

            print("Candidate:", candidate)
            print("Pipes:", [field.grid.get(c) for c in candidate])
            print("Len locs:", len(candidate))
            print()

            lengths[len(candidate)] = candidate

        print("Lengths:", lengths)
        print("Max length:", max(lengths.keys()))

        print("Solution path:")
        path = lengths[max(lengths.keys())]

        return path



field = Field(real_input)

print(field.grid)

print("Start:", field.start)


counter = 0
path = [field.start, (1, 2)]

path = field.get_path_from_start()

print(field.str_path(path))

print()
print(field)

print()

print(field.str_inside_out_path(path))