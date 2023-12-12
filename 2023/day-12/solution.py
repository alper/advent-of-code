from collections import deque
from typing import Deque, List, Tuple
import functools

sample_input = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""

real_input = sample_input
real_input = open("input.txt").read()

@functools.lru_cache(maxsize=None)
def check_possible(unknown: str, damaged: Tuple[int]) -> int:
    count = 0

    space = sum(damaged)
    limit = len(unknown) - space
    span = damaged[0]

    for i in range(limit+1):
        if i > 0 and unknown[i-1] == '#':
            break

        if all([c != '.' for c in unknown[i:i+span]]):
            if len(damaged) == 1:
                if all([c != '#' for c in unknown[i+span:]]):
                    count += 1
            elif (i+span == len(unknown) or unknown[i+span] != '#') and len(unknown) > i + space:
                count += check_possible(unknown[i+span+1:], damaged[1:])

    return count




s1 = 0
for line in real_input.splitlines():
    springs, broken = line.split()
    broken = [int(i) for i in broken.split(',')]

    print("Checking:", springs, broken)
    # s1 += check_possible(springs, "", springs, tuple(broken))

    c = check_possible(springs, tuple(broken))
    print("Possibilities:", c, "\n")
    s1 += c

print("Sum 1:", s1)

# Part 2

s2 = 0
for line in real_input.splitlines():
    springs, broken = line.split()
    springs = '?'.join([springs]*5)

    broken = [int(i) for i in broken.split(',')] * 5

    print("Checking:", springs, broken)
    possibilities = check_possible(springs, tuple(broken))

    print("Possibilities:", possibilities, "\n")
    s2 += possibilities

print("Sum 2:", s2)