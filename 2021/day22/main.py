import numpy as np


def parse_input(file_name):
    with open(file_name) as f:
        lines = f.readlines()

        l = []

        for line in lines:
            parts = line.strip().split()
            on_off = parts[0]

            coords = parts[1].split(",")
            x = (
                int(coords[0][2:].split("..")[0]),
                int(coords[0][2:].split("..")[1]),
            )
            y = (
                int(coords[1][2:].split("..")[0]),
                int(coords[1][2:].split("..")[1]),
            )
            z = (
                int(coords[2][2:].split("..")[0]),
                int(coords[2][2:].split("..")[1]),
            )

            l.append((on_off, x, y, z))

        return l


steps = parse_input("input.txt")

coords = {}

for step in steps:
    print(f"Step {step}")
    value = True if step[0] == "on" else False

    x_start, x_end = step[1][0], step[1][1]
    y_start, y_end = step[2][0], step[2][1]
    z_start, z_end = step[3][0], step[3][1]

    if (
        ((x_start >= -50 and x_start <= 50) or (x_end >= -50 and x_end <= 50))
        and ((y_start >= -50 and y_start <= 50) or (y_end >= -50 and y_end <= 50))
        and ((z_start >= -50 and z_start <= 50) or (z_end >= -50 and z_end <= 50))
    ):
        for x in range(x_start, x_end + 1):
            for y in range(y_start, y_end + 1):
                for z in range(z_start, z_end + 1):
                    if value:
                        coords[(x, y, z)] = True
                    else:
                        if (x, y, z) in coords:
                            del coords[(x, y, z)]

# n[:, :, 1:10] = True
print(len(coords))
