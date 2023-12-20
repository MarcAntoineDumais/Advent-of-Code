def load_data():
    file = open("data/day11.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def part1():
    grid = load_data()
    horizontal_expansions, vertical_expansions = get_expansions(grid)
    
    galaxies = []
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == "#":
                galaxies += [(x,y)]

    sum_lengths = 0
    for i in range(len(galaxies)):
        for i2 in range(i+1, len(galaxies)):
            x1, y1 = galaxies[i]
            x2, y2 = galaxies[i2]
            
            sum_lengths += abs(x1 - x2) + abs(y1 - y2)
            for exp in horizontal_expansions:
                if (x1 < exp and exp < x2) or (x2 < exp and exp < x1):
                    sum_lengths += 1
            for exp in vertical_expansions:
                if (y1 < exp and exp < y2) or (y2 < exp and exp < y1):
                    sum_lengths += 1

    print(f"{sum_lengths}")

def part2():
    grid = load_data()
    horizontal_expansions, vertical_expansions = get_expansions(grid)
    
    galaxies = []
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == "#":
                galaxies += [(x,y)]

    sum_lengths = 0
    for i in range(len(galaxies)):
        for i2 in range(i+1, len(galaxies)):
            x1, y1 = galaxies[i]
            x2, y2 = galaxies[i2]
            
            sum_lengths += abs(x1 - x2) + abs(y1 - y2)
            for exp in horizontal_expansions:
                if (x1 < exp and exp < x2) or (x2 < exp and exp < x1):
                    sum_lengths += 999999
            for exp in vertical_expansions:
                if (y1 < exp and exp < y2) or (y2 < exp and exp < y1):
                    sum_lengths += 999999

    print(f"{sum_lengths}")
     
def get_expansions(grid):
    vertical_expansions = []
    for y in range(len(grid)):
        if grid[y] == "." * len(grid[y]):
            vertical_expansions += [y]

    horizontal_expansions = []
    for x in range(len(grid[0])):
        empty = True
        for y in range(len(grid)):
            if grid[y][x] == "#":
                empty = False
                break
        if empty:
            horizontal_expansions += [x]

    return horizontal_expansions, vertical_expansions
