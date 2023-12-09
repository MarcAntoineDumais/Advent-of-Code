def load_data():
    file = open("data/day9.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def parse_data(data):
    return [[int(x) for x in line.split()] for line in data]

def part1():
    data = load_data()
    sequences = parse_data(data)

    total = 0
    for sequence in sequences:
        levels = [sequence]
        prev_level = sequence
        while len(levels) == 1 or sum([abs(x) for x in prev_level]) != 0:
            new_level = []
            for i in range(1, len(prev_level)):
                new_level += [prev_level[i] - prev_level[i-1]]
            prev_level = new_level
            levels += [new_level]
        levels = levels[:-1]

        val = 0
        for level in reversed(levels):
            val += level[-1]
        total += val

    print(f"{total}")

def part2():
    data = load_data()
    sequences = parse_data(data)

    total = 0
    for sequence in sequences:
        levels = [sequence]
        prev_level = sequence
        while len(levels) == 1 or sum([abs(x) for x in prev_level]) != 0:
            new_level = []
            for i in range(1, len(prev_level)):
                new_level += [prev_level[i] - prev_level[i-1]]
            prev_level = new_level
            levels += [new_level]
        levels = levels[:-1]

        val = 0
        for level in reversed(levels):
            val = level[0] - val
        total += val

    print(f"{total}")