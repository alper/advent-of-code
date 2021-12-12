from collections import defaultdict

class Graph:
    def __init__(self):
        self.graph = defaultdict(list)

        self.paths = []

    def add_edge(self, s):
        parts = s.split('-')

        self.graph[parts[0]].append(parts[1])
        self.graph[parts[1]].append(parts[0])

    def all_paths_rec(self, u, d, visited, path):
        if u.islower():
            visited[u] = True

        path.append(u)

        if u == d:
            self.paths.append(path)
        else:
            for i in self.graph[u]:
                if visited[i] == False:
                    self.all_paths_rec(i, d, visited, path)

        path.pop()
        visited[u] = False

    def all_paths(self, s, e):

        visited = defaultdict(lambda: False)
        path = []

        self.all_paths_rec(s, e, visited, path)

g = Graph()

with open('input.txt') as f:
    for line in f.readlines():
        print(line)
        g.add_edge(line.strip())

g.all_paths("start", "end")

print(len(g.paths))