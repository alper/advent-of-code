from collections import deque
from multiprocess import Pool
from copy import deepcopy

sample_input = r""".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."""

real_input = sample_input
real_input = open('input.txt').read()

class GridSpace:
	def __init__(self, m: str):
		self.mirror = m

		self.energized = 0
		self.beam_direction = ''

	def __str__(self):
		return self.mirror

DIR_MOVES = {
	'E': (1, 0),
	'S': (0, 1),
	'W': (-1, 0),
	'N': (0, -1)
}

DIR_CARET = {
	'E': '>',
	'S': 'v',
	'W': '<',
	'N': '^'
}

BACKSLASH_MIRROR = {'E': 'S', 'S': 'E', 'W': 'N', 'N': 'W'}
SLASH_MIRROR = {'E': 'N', 'S': 'W', 'W': 'S', 'N': 'E'}

class Grid:
	def __init__(self, inp: str):
		self.grid = {}

		for i, line in enumerate(inp.splitlines()):
			for j, el in enumerate(line):
				self.grid[(j, i)] = GridSpace(el)

		self.width = max([k[0] for k in self.grid.keys()]) + 1
		self.height = max([k[1] for k in self.grid.keys()]) + 1

	def __str__(self):
		l = []
		for y in range(self.height):
			for x in range(self.width):
				l.append(self.grid[(x, y)].mirror)
			l.append('\n')
		return ''.join(l)

	def beams_to_str(self):
		l = []
		for y in range(self.height):
			for x in range(self.width):
				# l.append(str(g[(x, y)].energized))
				if self.grid[(x, y)].beam_direction:
					l.append(self.grid[(x, y)].beam_direction)
				elif self.grid[(x, y)].mirror:
					l.append(self.grid[(x, y)].mirror)
				else:
					l.append(' ')
			l.append('\n')
		return ''.join(l)

	def cast_multiprocess(self, loc_dir) -> int:
		en_start = self.num_energized()

		res = self.cast_beam(loc_dir[0], loc_dir[1])

		print(f"Result: {res} for {loc_dir} ({en_start})")

		return res

	def cast_beam(self, loc, dir) -> int:
		beams = [(loc, dir)]

		energized_counts = [self.num_energized()]

		while True:
			if not beams:
				break

			# print(beams)

			new_beams = []

			for beam in beams:
				# Step the beam
				loc = beam[0]
				dir = beam[1]
				new_loc = (loc[0] + DIR_MOVES[dir][0], loc[1] + DIR_MOVES[dir][1])

				# print("New loc", new_loc)

				# Check the new location
				new_space = self.grid.get(new_loc)

				if not new_space:
					continue # Beam fell off the grid

				new_space.energized += 1

				if new_space.mirror == '\\':
					dir = BACKSLASH_MIRROR[dir]

					new_beams.append((new_loc, dir))
				elif new_space.mirror == '/':
					dir = SLASH_MIRROR[dir]

					new_beams.append((new_loc, dir))
				elif new_space.mirror == '|':
					if dir == 'E' or dir == 'W':
						new_beams.append((new_loc, 'N'))
						new_beams.append((new_loc, 'S'))
					else:
						new_beams.append((new_loc, dir))
				elif new_space.mirror == '-':
					if dir == 'N' or dir == 'S':
						new_beams.append((new_loc, 'E'))
						new_beams.append((new_loc, 'W'))
					else:
						new_beams.append((new_loc, dir))
				else:
					new_space.beam_direction = DIR_CARET[dir]
					new_beams.append((new_loc, dir))

			# print()
			# print(grid_beams_to_str(grid, width, height))
			# print()
			energized_counts.append(self.num_energized())
			# print("ENergized:", energized_counts[-1])

			# print("Length:", len(energized_counts))
			# print("Last count:", energized_counts[-50:])

			if len(energized_counts) > 200 and len(set(energized_counts[-50:])) <= 1:
				break
			# if new_energized == energized:
			# 	break


			# print("Energized:", grid_num_energized(grid))

			# Add the beam back to the list
			beams = new_beams
		return energized_counts[-1]


	def num_energized(self):
		return len([space for space in self.grid.values() if space.energized])

grid = Grid(real_input)
print(grid)

# beam_starts = [((-1, y), 'E') for y in range(grid.height)] + \
# 	[((x, -1), 'S') for x in range(grid.width)] + \
# 	[((grid.width, y), 'W') for y in range(grid.height)] + \
# 	[((x, grid.height), 'N') for x in range(grid.width)]

beam_starts_north = [((x, -1), 'S') for x in range(grid.width)]
beam_starts_west = [((-1, y), 'E') for y in range(grid.height)]
beam_starts_east = [((grid.width, y), 'W') for y in range(grid.height)]
beam_starts_south = [((x, grid.height), 'N') for x in range(grid.width)]

def check_beam(beam_start):
	grid = Grid(real_input)
	res = grid.cast_beam(beam_start[0], beam_start[1])
	return res

pool = Pool()
results = pool.map(check_beam, beam_starts_north + beam_starts_west + beam_starts_east + beam_starts_south)

# print("Solution:", max(results_north), max(results_south), max(results_east), max(results_west))
print("Solution:", max(results))

pool.close()
pool.join()

# loc = (3, -1)
# dir = 'S'

# print(grid.cast_beam(loc, dir))