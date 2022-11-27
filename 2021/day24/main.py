from multiprocessing import process
import multiprocessing


def parse(lines):
    steps = []

    for line in lines:
        parts = line.strip().split()
        instruction = parts[0]

        # print(parts)

        if instruction == "inp":
            steps.append((parts[0], parts[1]))
        else:
            left_op = parts[1]
            right_op = parts[2]

            if not right_op in ["w", "x", "y", "z"]:
                right_op = int(right_op)

            steps.append((instruction, left_op, right_op))

    return steps


def run(input_stack, program):
    registers = {
        "w": 0,
        "x": 0,
        "y": 0,
        "z": 0,
    }

    for step in program:
        instruction = step[0]
        if instruction == "inp":
            dest = step[1]
            registers[step[1]] = input_stack.pop()
        else:
            left = step[1]
            if isinstance(step[2], int):
                right = step[2]
            else:
                right = registers[step[2]]

            if instruction == "add":
                registers[left] = registers[left] + right
            elif instruction == "mul":
                registers[left] = registers[left] * right
            elif instruction == "div":
                registers[left] = registers[left] // right
            elif instruction == "mod":
                registers[left] = registers[left] % right
            elif instruction == "eql":
                registers[left] = registers[left] == right and 0 or 1

    return registers


def test_negater_program():
    negater = """inp x
mul x -1"""

    program = parse(negater.split("\n"))

    res = run([1], program)

    # print(res)

    assert res["x"] == -1


def test_triple_program():
    t_prog = """inp z
inp x
mul z 3
eql z x"""

    program = parse(t_prog.split("\n"))

    res = run([3, 9], program)

    assert res["z"] == 1


def test_binary_program():
    bin_prog = """inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"""

    program = parse(bin_prog.split("\n"))

    res = run([11], program)

    assert res["w"] == 1
    assert res["x"] == 0
    assert res["y"] == 1
    assert res["z"] == 1


def convert_number_to_stack(n):
    a = [int(digit) for digit in str(n)]
    a.reverse()
    return a


from functools import partial


def test_number(program, n):
    if n % 1000000 == 0:
        print(n)

    conv_number = convert_number_to_stack(n)

    if 0 in conv_number:
        return (n, False)

    res = run(conv_number, program)

    if res["z"] == 0:
        print("Found:", n)
        return (n, True)

    return (n, False)


from multiprocessing import Pool, freeze_support

if __name__ == "__main__":
    with open("input.txt") as f:
        lines = f.readlines()

    program = parse(lines)

    start_number = 99_999_999_999_999
    end_number = 11_111_111_111_111

    tester = partial(test_number, program)

    pool = Pool(15)

    for res in pool.imap(
        func=tester,
        iterable=range(start_number, end_number, -1),
        chunksize=100000,
    ):
        if res[1]:
            print("winner: ", res[0])

    print("After pool")
