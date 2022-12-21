from functools import reduce
import operator
import collections
from itertools import chain
from z3 import *

test_input = """root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""

test_input2 = """pppw == sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""


def get_input():
    i = test_input2
    i = open("full_input2.txt").read()

    return i.replace(":", " ==").split("\n")


def calculate_part1(puzzle_input):
    print("Calculating for", puzzle_input)

    s = Solver()

    # Get all variables
    for l in puzzle_input:
        v = l.split()[0]
        exec(f"{v} = Int('{v}')")

    for l in puzzle_input:
        s.add(eval(l))

    s.check()
    print(s)
    m = s.model()

    for d in m.decls():
        if d.name() == "root":
            print("%s = %s" % (d.name(), m[d]))

    return 1


def calculate_part2(puzzle_input):
    print("Calculating for", puzzle_input)

    s = Solver()

    humn = Int("humn")

    # Get all variables
    for l in puzzle_input:
        v = l.split()[0]
        exec(f"{v} = Int('{v}')")

    for l in puzzle_input:
        print("Add line", l)
        s.add(eval(l))

        # Need to add a l%r==0 constraint to get the right division
        if "/" in l:
            t, eq, l, op, r = l.split()
            s.add(eval(f"{l} % {r} == 0"))

    s.check()
    m = s.model()
    print(m)

    for d in m.decls():
        if d.name() == "humn":
            print("%s = %s" % (d.name(), m[d]))

    return 1


def part1(puzzle_input):
    return calculate_part1(puzzle_input)


def part2(puzzle_input):
    return calculate_part2(puzzle_input)


def test_calculation():
    assert calculate_part1(">") == false

    # assert calculate_part2("^v") == false


if __name__ == "__main__":
    puzzle_input = get_input()

    # print("Part 1 answer:", part1(puzzle_input))
    print("Part 2 answer:", part2(puzzle_input))
