
def calculate(i):
    return 0 + i.count('(') - i.count(')')

def get_input():
    return open('full_input.txt').read()

def part1(i):
    floor = calculate(i)
    print(floor)

def part2(i):
    floor = 0
    for index, c in enumerate(i):
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1

            if floor < 0:
                print(index+1)
                break


def test_calculation():
    assert calculate("(())") == 0


if __name__ == '__main__':
    i = get_input()

    part1(i)
    part2(i)