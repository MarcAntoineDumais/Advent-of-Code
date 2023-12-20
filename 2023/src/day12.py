from functools import cache

def load_data():
    file = open("data/day12.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def part1():
    data = [parse_line(line) for line in load_data()]
    data = [(split_string(x), y) for (x, y) in data]
    
    arrangements_sum = 0
    for line in data:
        arrangements_sum += solve(line[0], line[1])
    
    print(f"{arrangements_sum}")

def parse_line(line):
    parts = line.split()
    contiguous_groups = tuple(int(x) for x in parts[1].split(","))
    return (parts[0], contiguous_groups)

def part2():
    data = [parse_line(line) for line in load_data()]
    data = [unfold(x, y) for (x, y) in data]
    data = [(split_string(x), y) for (x, y) in data]
    
    arrangements_sum = 0
    for line in data:
        arrangements_sum += solve(line[0], line[1])
    
    print(f"{arrangements_sum}")
    
def unfold(string, contiguous_counts):
    string = "?".join([string] * 5)
    contiguous_counts = contiguous_counts * 5

    return string, contiguous_counts

def split_string(string):
    return tuple(x for x in string.split(".") if len(x) > 0)

@cache
def solve(string_groups, contiguous_counts):
    if len(contiguous_counts) == 0:
        for string in string_groups:
            for c in string:
                if c == "#":
                    return 0
        return 1
    
    if len(string_groups) == 0:
        return 0

    contiguous_count = contiguous_counts[0]
    contiguous_counts = contiguous_counts[1:]

    string = string_groups[0]
    string_groups = string_groups[1:]
    while len(string) < contiguous_count:
        if len(string_groups) == 0:
            return 0
        if "#" in string:
            return 0
        string = string_groups[0]
        string_groups = string_groups[1:]

    arrangements = solve(string_groups, (contiguous_count,) + contiguous_counts) if not "#" in string else 0
    
    i = 0
    while i < len(string) - contiguous_count + 1:
        if i > 0 and string[i - 1] == "#":
            break

        if len(string) > i+contiguous_count and string[i+contiguous_count] == "#":
            if string[i] == "#":
                break
            else:
                i += 1
                continue

        remaining_string = string[i+contiguous_count+1:]
        strings = string_groups if len(remaining_string) == 0 else (remaining_string,) + string_groups
        arrangements += solve(strings, contiguous_counts)

        i += 1

    return arrangements