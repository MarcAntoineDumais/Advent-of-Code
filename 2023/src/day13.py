def load_data():
    with open("data/day13.txt", "r") as file:
        data = file.read().splitlines()

    patterns = []
    pattern = []
    for line in data:
        if len(line.strip()) == 0:
            patterns += [pattern]
            pattern = []
            continue
        pattern += [line]
    if len(pattern) > 0:
        patterns += [pattern]

    return patterns

def part1():
    patterns = load_data()
    
    summary = 0
    for pattern in patterns:
        # vertical reflection
        mirror_index = find_mirror(pattern)
        if mirror_index != None:
            summary += 100 * (mirror_index + 1)

        # horizontal reflection
        transposed = transpose(pattern)
        mirror_index = find_mirror(transposed)
        if mirror_index != None:
            summary += mirror_index + 1

    print(f"{summary}")

def part2():
    patterns = load_data()
    
    summary = 0
    for pattern in patterns:
        transposed = transpose(pattern)
        mirror_index_vertical = find_mirror(pattern)
        mirror_index_horizontal = find_mirror(transposed)

        for y in range(len(pattern)):
            next_pattern = False
            for x in range(len(pattern[0])):
                prev = pattern[y][x]
                smudged = "." if prev == "#" else "#"
                pattern[y] = pattern[y][:x] + smudged + pattern[y][x+1:]
                transposed[x] = transposed[x][:y] + smudged + transposed[x][y+1:]

                new_index_vertical = find_mirror(pattern, exclude=mirror_index_vertical)
                new_index_horizontal = find_mirror(transposed, exclude=mirror_index_horizontal)

                if (new_index_vertical != mirror_index_vertical and new_index_vertical is not None) \
                    or (new_index_horizontal != mirror_index_horizontal and new_index_horizontal is not None):
                    if new_index_vertical != None and new_index_vertical != mirror_index_vertical:
                        summary += 100 * (new_index_vertical + 1)
                    if new_index_horizontal != None and new_index_horizontal != mirror_index_horizontal:
                        summary += new_index_horizontal + 1
                    next_pattern = True
                    break

                pattern[y] = pattern[y][:x] + prev + pattern[y][x+1:]
                transposed[x] = transposed[x][:y] + prev + transposed[x][y+1:]
            if next_pattern:
                break

    print(f"{summary}")

def transpose(pattern):
    transposed = []
    if len(pattern) == 0:
        return transposed
    
    for x in range(len(pattern[0])):
        line = ""
        for y in range(len(pattern)):
            line += pattern[y][x]
        transposed += [line]
    return transposed

def find_mirror(pattern, exclude=None):
    for i in range(len(pattern)-1):
        mirrored = True
        for j in range(min(i + 1, len(pattern) - i - 1)):
            if pattern[i - j] != pattern[i + j + 1]:
                mirrored = False
                break
        if mirrored:
            if i != exclude:
                return i

    return None