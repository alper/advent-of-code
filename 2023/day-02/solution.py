import math

sample_input = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

puzzle_input = sample_input
puzzle_input = open('input.txt').read()

# Part 1

bag = {'red': 12, 'green': 13, 'blue': 14}

possible_games_sum = 0

for line in puzzle_input.splitlines():
    print(line)

    # Parse the line
    game, rounds = line.split(':')
    _, game_number = game.split()

    rounds = [round.strip() for round in rounds.split(';')]

    print("Game: ", game)
    print("Rounds: ", rounds)

    game_possible = True

    # Parse the rounds
    for round in rounds:
        print("Round: ", round)

        # Parse the colors
        colors = round.split(',')

        for c in colors:
            count, color = c.split()

            if bag.get(color) < int(count):
                game_possible = False

    if game_possible:
        possible_games_sum += int(game_number)

print("Solution part 1: ", possible_games_sum)

# Part 2

power_sum = 0

for line in puzzle_input.splitlines():
    print(line)


    # Parse the line
    game, rounds = line.split(':')

    rounds = [round.strip() for round in rounds.split(';')]

    print("Game: ", game)
    # print("Rounds: ", rounds)

    maxes = {'red': 0, 'green': 0, 'blue': 0}

    for round in rounds:
        # print("Round: ", round)

        # Parse the colors
        colors = round.split(',')

        for c in colors:
            count, color = c.split()

            maxes[color] = max(maxes[color], int(count))

    print("Maxes: ", maxes)
    power = math.prod(maxes.values())

    print("Power: ", power)
    power_sum += power

print("Solution part 2: ", power_sum)