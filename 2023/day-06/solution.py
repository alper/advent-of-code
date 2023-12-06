sample_input = """Time:      7  15   30
Distance:  9  40  200"""

real_input = """Time:        41     96     88     94
Distance:   214   1789   1127   1055"""

puzzle_input = sample_input
puzzle_input = real_input

races = zip(*[list(map(lambda x: int(x.strip()), l.split(":")[1].split())) for l in real_input.split("\n")])

# print(list(races))

def number_of_possible_wins(total_time, max_distance):
    durations_pressed = range(total_time+1)

    distances = map(lambda t: t*(total_time-t), durations_pressed)

    wins = filter(lambda d: d > max_distance, distances)

    return len(list(wins))

wins_count = 1

for total_time, max_distance in races:
    print("Total time:", total_time, "Distance:", max_distance)

    wins = number_of_possible_wins(total_time, max_distance)

    wins_count *= wins

print("Part 1:", wins_count)

print("Part 2 sample:", number_of_possible_wins(71530, 940200))
print("Part 2 real:", number_of_possible_wins(41968894, 214178911271055))