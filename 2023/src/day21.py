import math

def load_data():
    with open("data/day21.txt", "r") as file:
        data = file.read().splitlines()

    return data

def part1():
    grid = load_data()
    
    start = None
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                start = (x, y)
                break
        if start is not None:
            break
    
    positions = set()
    positions.add(start)
    for _ in range(64):
        new_positions = set()
        for position in positions:
            # up
            if position[1] > 0 and grid[position[1] - 1][position[0]] != "#":
                new_positions.add((position[0], position[1]-1))
            # down
            if position[1] < len(grid) - 1 and grid[position[1] + 1][position[0]] != "#":
                new_positions.add((position[0], position[1]+1))
            # left
            if position[0] > 0 and grid[position[1]][position[0] - 1] != "#":
                new_positions.add((position[0] - 1, position[1]))
            # right
            if position[0] < len(grid[0]) - 1 and grid[position[1]][position[0] + 1] != "#":
                new_positions.add((position[0] + 1, position[1]))
        positions = new_positions

    print(f"{len(positions)}")

def part2():
    grid = load_data()
    start = None
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                start = (x, y)
                break
        if start is not None:
            break
        
    even_in_grid = 0
    odd_in_grid = 0
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] != "#":
                if (abs(y - start[1]) + abs(x - start[0])) % 2 == 0:
                    even_in_grid += 1
                else:
                    odd_in_grid += 1
    
    goal = 1000
    

def part2b():
    grid = load_data()
    
    start = None
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                start = (x, y)
                break
        if start is not None:
            break
    
    even_total = 0
    odd_total = 0
    for step in range(1, 5000+1):
        for i in range(-step, step+1):
            x = start[0] + i
            y1 = start[1] - (step - abs(i))
            y2 = start[1] + (step - abs(i))
            if grid[y1%len(grid)][x%len(grid[0])] != "#":
                if step%2 == 0:
                    even_total += 1
                else:
                    odd_total += 1
            if y1 != y2 and grid[y2%len(grid)][x%len(grid[0])] != "#":
                if step%2 == 0:
                    even_total += 1
                else:
                    odd_total += 1

    print(f"even: {even_total}  odd: {odd_total}")