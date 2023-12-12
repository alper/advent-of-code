from collections import deque
from typing import Deque

sample_input = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""

real_input = sample_input
# real_input = open("input.txt").read()

def check_possible(original: str, fixed: str, rest: str, conditions: Deque[int]):
    # print(f"Fixed {fixed} open {rest}, conditions {conditions}")

    conditions = conditions.copy()

    if not conditions:
        # We have placed all conditions, now we just need to check that the rest of the segment does not have damaged springs
        if '#' in rest:
            return

        fixed += len(rest) * '.'

        for i, c in enumerate(original):
            if c == '#' and fixed[i] != '#':
                return

        print(f"Found a possible solution: {fixed}")
        return

    if sum(conditions) > len(rest):
        return

    damaged_count = conditions.popleft()

    for i in range(len(rest)):
        chunk = rest[i:i+damaged_count]

        # Name the position indexes
        chunk_start = i
        chunk_end = i + damaged_count - 1

        chunk_after = None
        if chunk_end < len(rest) - 1:
            chunk_after = rest[chunk_end+1]

        if chunk.replace('?', '#') == "#" * damaged_count:
            if chunk_after == None:
                check_possible(original, fixed + ('#' * damaged_count), rest[chunk_end+1:], conditions)

            if chunk_after == "?" or chunk_after == ".":
                check_possible(original, fixed + ('#' * damaged_count) + '.', rest[chunk_end+2:], conditions)

            if chunk_after == "#":
                pass
        fixed += '.'
    return


def test_spring_possibilities():
    assert spring_possibilities("???.### 1,1,3") == 1
    assert spring_possibilities(".??..??...?##. 1,1,3") == 4
    assert spring_possibilities("?###???????? 3,2,1") == 10

# for line in real_input.splitlines():
#     springs, broken = line.split()
#     broken = [int(i) for i in broken.split(',')]

#     check_possible(springs, "", springs, deque(broken))

# Part 2

for line in real_input.splitlines():
    springs, broken = line.split()
    springs = '?'.join([springs]*5)

    broken = [int(i) for i in broken.split(',')] * 5

    check_possible(springs, "", springs, deque(broken))