#!/usr/bin/env python

DIE_ROLLS = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]


def part2():
    win1 = 0
    win2 = 0

    class WinsWrapper:
        def __init__(self):
            self.wins = 0

        def add(self, n):
            self.wins += n

    win1 = WinsWrapper()
    win2 = WinsWrapper()

    # dice(4, 0, win1, 8, 0, win2, 1) # sample
    dice(8, 0, win1, 6, 0, win2, 1)  # real

    print(f"Wins 1 {win1.wins} and wins 2 {win2.wins}")


def dice(pos1, score1, win1, pos2, score2, win2, score_mult):
    if score2 >= 21:
        win2.add(score_mult)

        print(f"Score added {score_mult}")
    else:
        for roll in DIE_ROLLS:
            new_pos = ((pos1 + roll[0] - 1) % 10) + 1

            dice(
                pos2,
                score2,
                win2,
                new_pos,
                score1 + new_pos,
                win1,
                score_mult * roll[1],
            )


part2()
