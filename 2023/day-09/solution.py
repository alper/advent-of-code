from typing import List
import itertools

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


def parse_line(line: str) -> List[int]:
    return [int(x) for x in line.split()]

def next_number(sequence: List[int]) -> int:
    print("Sequence:", sequence)

    sequences = [sequence]

    while True:
        new_sequence = [b-a for a, b in window(sequences[-1], 2)]
        print("Diff:", new_sequence)

        sequences.append(new_sequence)

        if all(x == 0 for x in new_sequence):
            break

    last_number = sequences[0][-1]

    # print(sequences)

    for i in range(len(sequences) - 1, 0, -1):
        l = sequences[i][-1]
        print("Adding", l)

        sequences[i-1].append(sequences[i-1][-1] + l)

    return sequences[0][-1]

l = [parse_line(line) for line in real_input.splitlines()]

total = 0

print("DEBUG")

for x in l:
    n = next_number(x)

    print(f"[{n}] number: {x}")
    total += n

# n = [next_number(x) for x in l]

print("Solution:", total)

print("DEBUG SPECIFIC")

p = parse_line("12 16 26 51 103 202 405 870 1965 4434 9640 19935 39296 74578 138165 253576 466866 868644 1634430 3095131 5854885")
n = next_number(p)
print("Optupt", n)

# print(next_number(l[0]))