import string

sample_input = '''1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet'''

puzzle_input = open('input.txt', 'r').read()

# Part 1

def part1():
    text = puzzle_input

    digits = [[c for c in l if c in string.digits] for l in text.split('\n')]

    print(digits)

    numbers = [int(n[0])*10 + int(n[-1]) for n in digits]

    print(numbers)

    print("Solution part 1: ", sum(numbers))

# Part 2

sample_input = '''two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen'''

text = puzzle_input

mapping = {
    'one': '1',
    'two': '2',
    'three': '3',
    'four': '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9'
}

def left_written_digit(s):
    print("Left written:", s)
    for i in range(len(s)):
        for k, v in mapping.items():
            print(s[i:], k, v)
            if s[i:].startswith(k):
                print("Starts with ", k)
                return v
    return None

def right_written_digit(s):
    for i in range(len(s)-1, 0, -1):
        for k, v in mapping.items():
            if s[i:].startswith(k):
                return v
    return None



def get_written_digits(s):
    print("Got string:", s)

    # Pass from the left
    left_digit = None
    for i in range(len(s)):
        print("i: ", i, "s[i]: ", s[i])

        if s[i] in string.digits:
            # Check if we don't have a written digit before
            left_digit = left_written_digit(s[:i]) or int(s[i])
            break

    if left_digit == None:
        left_digit = left_written_digit(s)

    print("Left digit", left_digit)

    right_digit = None
    for i in range(len(s)-1, -1, -1):
        print("i: ", i, "s[i]: ", s[i])

        if s[i] in string.digits:
            # Check if we don't have a written digit after
            right_digit = right_written_digit(s[i:]) or int(s[i])
            break

    if right_digit == None:
        right_digit = right_written_digit(s)

    print("Right digit", right_digit)

    return (left_digit, right_digit)

sum = 0

for l in text.split('\n'):
    r = get_written_digits(l)
    print(l)
    print(r)
    print()

    sum += int(r[0])*10 + int(r[1])

print("Solution part 2: ", sum)
