import heapq
from copy import deepcopy

sample_input = """2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"""

real_input = sample_input

grid = {}

for i, l in enumerate(real_input.splitlines()):
    for j, c in enumerate(l):
        grid[(j, i)] = int(c)

start = (0, 0)
end = (12, 12)

class Path:
    def __init__(self, points):
        self.points = points
        self.cost = 0

    def __lt__(self, other):
        return self.cost < other.cost

def find(start, end, grid):
    paths = []
    heapq.heappush(paths, Path([start]))

    # visited = set()

    while paths:
        path = heapq.heappop(paths)
        seen = set()

        print("Path", path.cost)

        x, y = path.points[-1]


        if (x, y) == end:
            return path

        if (x, y) in seen:
            continue
        seen.add((x, y))

        # if (x, y) in visited:
        #     continue

        # visited.add((x, y))

        for adj in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            new_x = x + adj[0]
            new_y = y + adj[1]

            if grid.get((new_x, new_y)):
                new_path = deepcopy(path)
                new_path.points.append((new_x, new_y))
                new_path.cost += grid[(new_x, new_y)]

                heapq.heappush(paths, new_path)

    return None

find(start, end, grid)