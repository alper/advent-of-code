from functools import reduce
import operator
import collections
from itertools import chain
import pyparsing as pp
import ctypes


def parse_input(puzzle_input):
    wire_name = pp.Word(pp.nums) ^ pp.Word(pp.alphas)
    assignment = wire_name + pp.Word("->") + pp.Word(pp.alphas)
    and_op = wire_name + pp.Word("AND") + wire_name + pp.Word("->") + wire_name
    or_op = wire_name + pp.Word("OR") + wire_name + pp.Word("->") + wire_name
    lshift_op = wire_name + pp.Word("LSHIFT") + wire_name + pp.Word("->") + wire_name
    rshift_op = wire_name + pp.Word("RSHIFT") + wire_name + pp.Word("->") + wire_name
    not_op = pp.Word("NOT") + wire_name + pp.Word("->") + wire_name

    instruction_parser = assignment ^ and_op ^ or_op ^ lshift_op ^ rshift_op ^ not_op

    r = []
    for line in puzzle_input.split("\n"):
        print("Parsing", line)
        p = instruction_parser.parse_string(line)
        print(p)
        r.append(p)

    return r


def get_test_input():
    test_input = """123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"""

    return parse_input(test_input)


def get_input():
    return parse_input(open("full_input.txt").read())


def calculate_part1(puzzle_input):
    vals = collections.deque()

    # Find the initial assignments
    for i in reversed(range(len(puzzle_input))):
        ins = puzzle_input[i]

        if ins[1] == "->":
            print(i)
            try:
                l_num = int(ins[0])

                assignment = (ins[2], l_num)
                print("Creating assignment: ", assignment)

                vals.append(assignment)
                del puzzle_input[i]
            except:
                pass

    print("Initial assignments:", vals)

    # Replace the assignments in the other instructions
    print("Replacing")
    while vals:
        replace_val = vals.popleft()

        print("Replace val", replace_val)

        for i in reversed(range(len(puzzle_input))):
            ins = puzzle_input[i]

            print("Candidate", ins)
            if not ins:
                continue

            if ins[1] == "->":
                if ins[0] == replace_val[0]:
                    assignment = (ins[2], replace_val[1])

                    print("Creating assignment: ", assignment)
                    vals.append(assignment)

                    del puzzle_input[i]
            elif ins[0] == "NOT" and ins[1] == replace_val[0]:
                assignment = (ins[3], ctypes.c_uint16(~replace_val[1]).value)

                print("Creating assignment: ", assignment)
                vals.append(assignment)
                del puzzle_input[i]
            elif ins[0] == replace_val[0] or ins[2] == replace_val[0]:
                if ins[0] == replace_val[0]:
                    ins[0] = replace_val[1]
                elif ins[2] == replace_val[0]:
                    ins[2] = replace_val[1]

                try:
                    l_num = int(ins[0])
                    r_num = int(ins[2])

                    print("Processing ins: ", ins)

                    if ins[1] == "AND":
                        assignment = (ins[4], l_num & r_num)
                    elif ins[1] == "OR":
                        assignment = (ins[4], l_num | r_num)
                    elif ins[1] == "LSHIFT":
                        assignment = (ins[4], l_num << r_num)
                    elif ins[1] == "RSHIFT":
                        assignment = (ins[4], l_num >> r_num)

                    print("Creating assignment: ", assignment)

                    vals.append(assignment)
                    del puzzle_input[i]
                except:
                    print("Replaced:", ins)

    print("Remainder: ", puzzle_input)

    return 1


def calculate_part2(puzzle_input):
    vals = collections.deque()

    # Find the initial assignments
    for i in reversed(range(len(puzzle_input))):
        ins = puzzle_input[i]

        if ins[0] == "1674":
            puzzle_input[i] = ["46065", "->", "b"]
            ins = ["46065", "->", "b"]

        if ins[1] == "->":
            print(i)
            try:
                l_num = int(ins[0])

                assignment = (ins[2], l_num)
                print("Creating assignment: ", assignment)

                vals.append(assignment)
                del puzzle_input[i]
            except:
                pass

    print("Initial assignments:", vals)

    # Replace the assignments in the other instructions
    print("Replacing")
    while vals:
        replace_val = vals.popleft()

        print("Replace val", replace_val)

        for i in reversed(range(len(puzzle_input))):
            ins = puzzle_input[i]

            print("Candidate", ins)
            if not ins:
                continue

            if ins[1] == "->":
                if ins[0] == replace_val[0]:
                    assignment = (ins[2], replace_val[1])

                    print("Creating assignment: ", assignment)
                    vals.append(assignment)

                    del puzzle_input[i]
            elif ins[0] == "NOT" and ins[1] == replace_val[0]:
                assignment = (ins[3], ctypes.c_uint16(~replace_val[1]).value)

                print("Creating assignment: ", assignment)
                vals.append(assignment)
                del puzzle_input[i]
            elif ins[0] == replace_val[0] or ins[2] == replace_val[0]:
                if ins[0] == replace_val[0]:
                    ins[0] = replace_val[1]
                elif ins[2] == replace_val[0]:
                    ins[2] = replace_val[1]

                try:
                    l_num = int(ins[0])
                    r_num = int(ins[2])

                    print("Processing ins: ", ins)

                    if ins[1] == "AND":
                        assignment = (ins[4], l_num & r_num)
                    elif ins[1] == "OR":
                        assignment = (ins[4], l_num | r_num)
                    elif ins[1] == "LSHIFT":
                        assignment = (ins[4], l_num << r_num)
                    elif ins[1] == "RSHIFT":
                        assignment = (ins[4], l_num >> r_num)

                    print("Creating assignment: ", assignment)

                    vals.append(assignment)
                    del puzzle_input[i]
                except:
                    print("Replaced:", ins)

    print("Remainder: ", puzzle_input)

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

    print(puzzle_input)

    # print("Part 1 answer:", part1(puzzle_input))
    print("Part 2 answer:", part2(puzzle_input))
