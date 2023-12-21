from collections import deque, OrderedDict

sample_input = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"""

real_input = sample_input
real_input = open('input.txt', 'r').read()

def hash(inp: str) -> int:
	current_value = 0
	
	for c in inp:
		current_value += ord(c)
		current_value *= 17
		current_value %= 256

	return current_value

print(hash("HASH"))

res = [hash(seg) for seg in real_input.strip().split(',')]

print(sum(res))

# Part 2

boxes = OrderedDict()
for i in range(0, 256):
	boxes[i] = deque()

print(boxes)

def label_index(label, box):
	hit_index = -1
	for i, lens in enumerate(box):
		if lens[0] == label:
			hit_index = i
			break
	return hit_index

for instruction in real_input.strip().split(','):
	if '=' in instruction:
		label = instruction.split('=')[0]
		box_number = hash(label)

		focal = int(instruction.split('=')[1])

		hit_index = label_index(label, boxes[box_number])

		if hit_index != -1:
			# Found it in the list so replace it
			boxes[box_number][hit_index] = (label, focal)
		else:
			boxes[box_number].append((label, focal))
	elif '-' in instruction:
		label = instruction.split('-')[0]
		box_number = hash(label)

		hit_index = label_index(label, boxes[box_number])

		if hit_index != -1:
			del boxes[box_number][hit_index]


focusing_power = 0

for k in boxes.keys():
	print(k, boxes[k])

	for i, lens in enumerate(boxes[k]):
		focusing_power += (k+1) * (i+1) * lens[1]
		

print("Total power", focusing_power)
