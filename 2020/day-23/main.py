from collections import deque


def part1():
    test_input = "389125467"
    real_input = "326519478"
    test_input = real_input

    cups = deque([int(c) for c in test_input])

    MAX_CUP = max(cups)
    MIN_CUP = min(cups)

    # The current cup is always the 0 index element in the deque

    for move in range(1, 101):
        print(f"-- move {move} --")
        print(cups)

        current_cup = cups.popleft()

        picked_up = [cups.popleft(), cups.popleft(), cups.popleft()]

        print(f"pick up: {picked_up}")

        cups.appendleft(current_cup)

        destination_candidate = current_cup - 1
        while True:
            try:
                destination_index = cups.index(destination_candidate)

                print(f"dstination: {destination_candidate}")

                cups.insert(destination_index + 1, picked_up[0])
                cups.insert(destination_index + 2, picked_up[1])
                cups.insert(destination_index + 3, picked_up[2])
                break
            except ValueError:
                destination_candidate -= 1

                if destination_candidate < MIN_CUP:
                    destination_candidate = MAX_CUP

        cups.rotate(-1)

        print("")
        # print(f"After inserting: {cups}")

    print(f"Cups after 100 moves: {cups}")

    while cups[0] != 1:
        cups.rotate(-1)

    cups.popleft()
    print(f"Answer: {''.join([str(c) for c in cups])}")


def part2():
    test_input = "389125467"
    real_input = "326519478"
    test_input = real_input

    cups = deque([int(c) for c in test_input])

    MAX_INPUT_CUP = max(cups)
    MIN_INPUT_CUP = min(cups)

    for cup_value in range(MAX_INPUT_CUP + 1, 1_000_001):
        cups.append(cup_value)

    MIN_CUP = MIN_INPUT_CUP
    MAX_CUP = 1_000_000

    # The current cup is always the 0 index element in the deque

    for move in range(1, 10_000_001):
        print(f"-- move {move} --")
        # print(cups)

        current_cup = cups.popleft()

        picked_up = [cups.popleft(), cups.popleft(), cups.popleft()]

        # print(f"pick up: {picked_up}")

        cups.appendleft(current_cup)

        destination_candidate = current_cup - 1
        while True:
            try:
                destination_index = cups.index(destination_candidate)

                # print(f"destination: {destination_candidate}")

                cups.insert(destination_index + 1, picked_up[0])
                cups.insert(destination_index + 2, picked_up[1])
                cups.insert(destination_index + 3, picked_up[2])
                break
            except ValueError:
                destination_candidate -= 1

                if destination_candidate < MIN_CUP:
                    destination_candidate = MAX_CUP

        cups.rotate(-1)

        # print("")
        # print(f"After inserting: {cups}")

    print(f"Cups after 10'000'000 moves: {cups}")

    index_of_one = cups.index(1)

    print(f"Anser part2: {cups[index_of_one+1] * cups[index_of_one+2]}")


part2()
