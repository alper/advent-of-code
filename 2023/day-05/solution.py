import itertools

sample_input = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""

puzzle_input = sample_input
puzzle_input = open("input.txt").read()

elements = puzzle_input.split("\n\n")

seeds = [int(n) for n in elements[0].strip().split(": ")[1].split(" ")]

print(seeds)

mappings = []

for mapping in elements[1:]:
    m = []

    mapping = mapping.strip().split("\n")
    fr, _, to = mapping[0].split()[0].split("-")

    print("fr:", fr, "to:", to)

    for row in mapping[1:]:
        dest_start, source_start, length = [int(n) for n in row.split()]

        print("dest_start:", dest_start, "source_start:", source_start, "length:", length)

        m.append((source_start, source_start+length-1, dest_start, dest_start+length-1))

    print()
    mappings.append(m)

print(mappings)

locations = []

for seed in seeds:
    # print("Seed:", seed)

    current_number = seed

    for mapping in mappings:
        for source_start, source_end, dest_start, dest_end in mapping:
            if current_number in range(source_start, source_end+1):
                current_number = dest_start + (current_number - source_start)
                # print("New number:", current_number)
                break

    location = current_number
    locations.append(location)
    # print("Location: ", location)

print("Solution part 1:", min(locations))

def range_to_ranges(r, mappings):
    print("R ", r)
    start, end = r
    dest_ranges = []

    while True:
        hitFound = False

        for source_start, source_end, dest_start, dest_end in mappings:
            if start >= source_start and end <= source_end:
                # Fully enclosed
                dest_ranges.append((dest_start + (start - source_start), dest_start + (end - source_start)))

                start = 0
                end = 0

                hitFound = True
            elif start < source_start and end >= source_start and end <= source_end:
                # Takes left of mapping range
                dest_ranges.append((dest_start, dest_start + (end - source_start)))

                end = source_start - 1
                hitFound = True
            elif start >= source_start and start <= source_end and end > source_end:
                # Takes right part of mapping range
                dest_ranges.append((
                    dest_start + (start - source_start),
                    dest_end))

                start = source_end + 1
                hitFound = True
            else:
                # No mapping found
                pass

        if not hitFound:
            # print("Hit not found", start, end)
            if start != 0 and end != 0:
                dest_ranges.append((start, end))
            break

        if start == 0 and end == 0:
            break

    print("to", dest_ranges)
    return dest_ranges

print()

seed_ranges = [(b[0], b[0]+b[1]-1) for b in itertools.batched(seeds, 2)]
print("Seed ranges:", seed_ranges)
print()

for i, mapping in enumerate(mappings):
    print("Mapping", i)
    print("Mapping:", [f"{m[0]}-{m[1]} to {m[2]}-{m[3]}" for m in sorted(mapping)])
    seed_ranges = list(itertools.chain.from_iterable(map(lambda r: range_to_ranges(r, mapping), seed_ranges)))
    print()

print("Seed ranges:", seed_ranges)

print("Solution:", min(min(seed_ranges)))

# Bug
# R  (57, 69)
# to [(53, 52), (61, 69)]