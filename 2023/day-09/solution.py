from typing import List
import itertools
from collections import deque

sample_input = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""

real_input = sample_input
real_input = open("input.txt").read()

def window(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(itertools.islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result


def parse_line(line: str) -> deque[int]:
    return deque([int(x) for x in line.split()])

def next_and_previous_number(sequence: deque[int]) -> (int, int):
    print("Sequence:", sequence)

    sequences = [sequence]

    while True:
        new_sequence = deque([b-a for a, b in window(sequences[-1], 2)])
        print("Diff:", new_sequence)

        sequences.append(new_sequence)

        if all(x == 0 for x in new_sequence):
            break

    # print(sequences)

    for i in range(len(sequences) - 1, 0, -1):
        last = sequences[i][-1]
        first = sequences[i][0]

        # print("Adding", l)

        sequences[i-1].append(sequences[i-1][-1] + last)
        sequences[i-1].appendleft(sequences[i-1][0] - first)

    return (sequences[0][0], sequences[0][-1])

parsed = [parse_line(line) for line in real_input.splitlines()]

total = 0

print("DEBUG")

next_and_previous_number = [next_and_previous_number(l) for l in parsed]

print("Solution part 1:", sum([l for f, l in next_and_previous_number]))

print("Solution part 2:", sum([f for f, l in next_and_previous_number]))
