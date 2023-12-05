def load_data():
    file = open("data/day5.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def parse_data(data):
    seeds = [int(x) for x in data[0].split(":")[1].split()]

    i = 3
    maps = []
    cur_map = []
    while i < len(data):
        if data[i] == "" or i == len(data) - 1:
            maps += [cur_map]
            cur_map = []
            i += 1
        else:
            cur_map += [[int(x) for x in data[i].split()]]
        i += 1

    return seeds, maps


def part1():
    data = load_data()
    seeds, maps = parse_data(data)

    min_location = None
    for seed in seeds:
        location = seed
        #print(f"seed {seed}")
        for map in maps:
            location = follow_map(location, map)
            #print(location)
        min_location = location if min_location is None else min(location, min_location)

    print(min_location)

def part2():
    data = load_data()
    seeds, maps = parse_data(data)
    seed_ranges = []
    for i in range(len(seeds)):
        if i%2 == 0:
            seed_ranges += [(seeds[i], seeds[i+1])]

    min_location = None
    for seed_range in seed_ranges:
        ranges = [seed_range]
        
        for map in maps:
            ranges = follow_map_ranges(ranges, map)
        for r in ranges:
            min_location = r[0] if min_location is None else min(r[0], min_location)

    print(min_location)

def follow_map(x, map):
    for row in map:
        if x >= row[1] and x < row[1] + row[2]:
            return row[0] + x - row[1]
    return x

def follow_map_ranges(ranges, map):
    boundaries = set()
    for row in map:
        boundaries.add(row[1])
        boundaries.add(row[1] + row[2])
    boundaries = sorted(boundaries)

    split_ranges = []
    while len(ranges) > 0:
        r = ranges.pop()
        broken = False
        for boundary in boundaries:
            overlap = boundary - r[0]
            if in_interval(boundary, r[0], r[1]) and overlap > 0:
                split_ranges += [(r[0], overlap)]
                ranges += [(boundary, r[1] - overlap)]
                broken = True
                break
        if not broken:
            split_ranges += [r]
        
    new_ranges = []
    for r in split_ranges:
        added = False
        for row in map:
            if in_interval(r[0], row[1], row[2]):
                new_ranges += [(row[0] + r[0] - row[1], r[1])]
                added = True
                break
        if not added:
            new_ranges += [r]
    return new_ranges

def in_interval(x, start, size):
    return x >= start and x < start + size