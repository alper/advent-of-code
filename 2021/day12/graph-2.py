from collections import defaultdict

class Graph:
    def __init__(self):
        self.graph = defaultdict(list)

        self.paths = []

    def add_edge(self, s):
        parts = s.split('-')

        self.graph[parts[0]].append(parts[1])
        self.graph[parts[1]].append(parts[0])

    def all_paths_rec(self, u, visited, path, twice=False):
        if u == 'end':
            self.paths.append(path + ['end'])
            return

        if u.islower() and u in visited:
            if twice or u == 'start' or u == 'end':
                return
            else:
                twice = True

        for e in self.graph[u]:
            self.all_paths_rec(e, path + [u], visited + [u], twice)

g = Graph()

with open('input.txt') as f:
    for line in f.readlines():
        # print(line)
        g.add_edge(line.strip())


g.all_paths_rec('start', [], [], False)

print(g.paths)
print(len(g.paths))