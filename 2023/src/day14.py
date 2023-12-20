from utils import Direction

def load_data():
    with open("data/day14.txt", "r") as file:
        data = file.read().splitlines()
        data = [list(x) for x in data]
    return data

def part1():
    grid = load_data()
    
    tilt(grid, Direction.NORTH)
    total_load = calculate_load(grid)

    print(f"{total_load}")

def tilt(grid, direction):
    match direction:
        case Direction.NORTH:
            for y in range(len(grid)):
                for x in range(len(grid[y])):
                    if grid[y][x] != "O":
                        continue
                    new_y = y - 1
                    while new_y >= 0 and grid[new_y][x] == ".":
                        grid[new_y][x] = "O"
                        grid[new_y + 1][x] = "."
                        new_y -= 1
        case Direction.EAST:
            for x in reversed(range(len(grid[0]))):
                for y in range(len(grid)):
                    if grid[y][x] != "O":
                        continue
                    new_x = x + 1
                    while new_x < len(grid[y]) and grid[y][new_x] == ".":
                        grid[y][new_x] = "O"
                        grid[y][new_x - 1] = "."
                        new_x += 1
        case Direction.SOUTH:
            for y in reversed(range(len(grid))):
                for x in range(len(grid[y])):
                    if grid[y][x] != "O":
                        continue
                    new_y = y + 1
                    while new_y < len(grid) and grid[new_y][x] == ".":
                        grid[new_y][x] = "O"
                        grid[new_y - 1][x] = "."
                        new_y += 1
        case Direction.WEST:
            for x in range(len(grid[0])):
                for y in range(len(grid)):
                    if grid[y][x] != "O":
                        continue
                    new_x = x - 1
                    while new_x >= 0 and grid[y][new_x] == ".":
                        grid[y][new_x] = "O"
                        grid[y][new_x + 1] = "."
                        new_x -= 1
            

def calculate_load(grid):
    total_load = 0
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "O":
                total_load += len(grid) - y
    return total_load

def part2():
    grid = load_data()
    
    goal = 1000000000
    last_cycle = {}
    cycle = 0
    while cycle < goal:
        key = tuple([tuple(x) for x in grid])
        if key in last_cycle:
            val = last_cycle[key]
            cycle_size = cycle - val

            skipping_cycles = ((goal - cycle) // cycle_size) * cycle_size
            print(f"Cycle of size {cycle_size} detected: skipping {skipping_cycles} cycles")

            cycle += skipping_cycles
            
            del last_cycle[key]
            continue

        tilt(grid, Direction.NORTH)
        tilt(grid, Direction.WEST)
        tilt(grid, Direction.SOUTH)
        tilt(grid, Direction.EAST)
        last_cycle[key] = cycle
        cycle += 1
    
    total_load = calculate_load(grid)
    
    print(f"{total_load}")
    