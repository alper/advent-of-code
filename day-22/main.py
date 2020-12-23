puzzle_input = open("test_input.txt").read()
# puzzle_input = open("input.txt").read()


def part1():
    players = [
        [int(c) for c in player.strip().split("\n")[1:]]
        for player in puzzle_input.split("\n\n")
    ]

    print(players)

    round = 1

    while players[0] and players[1]:
        print(f"Round {round}")

        card0 = players[0].pop(0)
        card1 = players[1].pop(0)

        print(f"Player 1 plays: {card0}")
        print(f"Player 2 plays: {card1}")

        if card0 > card1:
            print("Player 1 wins")
            players[0].append(card0)
            players[0].append(card1)
        elif card1 > card0:
            print("Player 2 wins")
            players[1].append(card1)
            players[1].append(card0)

        print(f"Player 1 deck: {players[0]}")
        print(f"Player 2 deck: {players[1]}")

        round += 1

    winning_deck = players[0] or players[1]

    score = sum(
        map(
            lambda ic: ic[0] * ic[1],
            zip(range(1, len(winning_deck) + 1), reversed(winning_deck)),
        )
    )

    print(f"Score of the winner is: {score}")


def part2():
    players = [
        [int(c) for c in player.strip().split("\n")[1:]]
        for player in puzzle_input.split("\n\n")
    ]

    def play_recursive(deck1, deck2, level=1):
        print(f"Playing recursive game {level}")
        print(f"Decks: {deck1} x {deck2}")

        player1_states = []
        player2_states = []

        rounds = 1
        while deck1 and deck2:
            print(f"Round {rounds}")

            # Check if deck in previous state
            # if deck1 in player1_states or deck2 in player2_states:
            #     print(f"States {player1_states} x {player2_states}")
            #     return (deck1, [])
            # else:
            #     player1_states.append(deck1)
            #     player2_states.append(deck2)

            card1 = deck1.pop(0)
            card2 = deck2.pop(0)

            print(f"Card player 1: {card1}")
            print(f"Card player 2: {card2}")

            if len(deck1) >= card1 and len(deck2) >= card2:
                winner, wdeck1, wdeck2 = play_recursive(
                    deck1[:], deck2[:], level + 1
                )
            else:
                print("Playing non recursive")
                if card1 > card2:
                    winner = 0
                elif card2 > card1:
                    winner = 1

            print(f"Winner is: {winner+1}")

            if winner == 0:
                deck1.append(card1)
                deck1.append(card2)
            else:
                deck2.append(card2)
                deck2.append(card1)

            print(f"Decks: {deck1} x {deck2}")
            print(f"------------ End round {rounds}")

            rounds += 1

        print(f"Decks: {deck1} x {deck2}")
        print(f"------------ End recursive game {level}")

        return (winner, deck1, deck2)

    winner, wdeck1, wdeck2 = play_recursive(players[0][:], players[1][:])

    score = sum(
        map(
            lambda ic: ic[0] * ic[1],
            zip(
                range(1, len(wdeck1 or wdeck2) + 1),
                reversed(wdeck1 or wdeck2),
            ),
        )
    )

    print(f"Winner part 2 is {winner+1}")
    print(f"Score: {score}")


part2()