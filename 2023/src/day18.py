import utils

def load_data():
    with open("data/day18.txt", "r") as file:
        data = file.read().splitlines()
    return data

def part1():
    steps = [parse_line(line) for line in load_data()]
    area = calculate_area(steps)
    print(area)

def parse_line(line):
    parts = line.split()
    dir = None
    match parts[0]:
        case "U":
            dir = utils.Direction.NORTH
        case "R":
            dir = utils.Direction.EAST
        case "D":
            dir = utils.Direction.SOUTH
        case "L":
            dir = utils.Direction.WEST
    return dir, int(parts[1])

def calculate_area(steps):
    pos = (0, 0)
    loop = [pos]
    for step in steps[:-1]:
        incr = utils.step_increment(step[0])
        pos = (pos[0] + incr[0] * step[1], pos[1] + incr[1] * step[1])
        loop += [pos]

    # Adjust vertices to be the outside corners of the loop
    adjusted_loop = []
    for i in range(len(loop)):
        corner = set([utils.get_direction(loop[i-1], loop[i]), utils.get_direction(loop[i], loop[(i+1)%len(loop)])])
        adjustment = (0, 0)
        if utils.Direction.NORTH in corner:
            if utils.Direction.EAST in corner:
                adjustment = (0, 0)
            else:
                adjustment = (0, 1)
        else:
            if utils.Direction.EAST in corner:
                adjustment = (1, 0)
            else:
                adjustment = (1, 1)

        adjusted_loop += [(loop[i][0] + adjustment[0], loop[i][1] + adjustment[1])]
    loop = adjusted_loop

    # Calculate area
    area = 0
    prev = loop[-1]
    for cur in loop:
        area += (prev[0] + cur[0]) * (prev[1] - cur[1])
        prev = cur
    return abs(area // 2)

def part2():
    steps = [parse_line_2(line) for line in load_data()]
    area = calculate_area(steps)
    print(area)

def parse_line_2(line):
    parts = line.split()
    dir = None
    match parts[2][-2]:
        case "0":
            dir = utils.Direction.EAST
        case "1":
            dir = utils.Direction.SOUTH
        case "2":
            dir = utils.Direction.WEST
        case "3":
            dir = utils.Direction.NORTH
    return dir, int(parts[2][2:7], 16)
