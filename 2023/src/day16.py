from utils import Direction

def load_data():
    with open("data/day16.txt", "r") as file:
        data = file.read().splitlines()
    return data

def part1():
    grid = load_data()
    
    energized = set((0, 0))
    seen = set((0, 0, Direction.EAST))
    todo = [(0, 0, Direction.EAST)]
    while len(todo) > 0:
        cur = todo.pop()
        next_positions = step(grid, cur[0], cur[1], cur[2])
        for next_position in next_positions:
            if next_position not in seen:
                seen.add(next_position)
                todo += [next_position]
                energized.add(next_position[:2])

    print(f"{len(energized)}")

def step(grid, x, y, direction):
    next_positions = []
    match direction:
        case Direction.NORTH:
            match grid[y][x]:
                case "." | "|":
                    next_positions += [(x, y-1, Direction.NORTH)]
                case "-":
                    next_positions += [(x+1, y, Direction.EAST), (x-1, y, Direction.WEST)]
                case "/":
                    next_positions += [(x+1, y, Direction.EAST)]
                case "\\":
                    next_positions += [(x-1, y, Direction.WEST)]
        case Direction.SOUTH:
            match grid[y][x]:
                case "." | "|":
                    next_positions += [(x, y+1, Direction.SOUTH)]
                case "-":
                    next_positions += [(x+1, y, Direction.EAST), (x-1, y, Direction.WEST)]
                case "/":
                    next_positions += [(x-1, y, Direction.WEST)]
                case "\\":
                    next_positions += [(x+1, y, Direction.EAST)]
        case Direction.EAST:
            match grid[y][x]:
                case "." | "-":
                    next_positions += [(x+1, y, Direction.EAST)]
                case "|":
                    next_positions += [(x, y-1, Direction.NORTH), (x, y+1, Direction.SOUTH)]
                case "/":
                    next_positions += [(x, y-1, Direction.NORTH)]
                case "\\":
                    next_positions += [(x, y+1, Direction.SOUTH)]
        case Direction.WEST:
            match grid[y][x]:
                case "." | "-":
                    next_positions += [(x-1, y, Direction.WEST)]
                case "|":
                    next_positions += [(x, y-1, Direction.NORTH), (x, y+1, Direction.SOUTH)]
                case "/":
                    next_positions += [(x, y+1, Direction.SOUTH)]
                case "\\":
                    next_positions += [(x, y-1, Direction.NORTH)]
    
    return [pos for pos in next_positions if is_in_grid(grid, pos[0], pos[1])]



def is_in_grid(grid, x, y):
    return x >= 0 and x < len(grid[0]) and y >= 0 and y < len(grid)

def part2():
    grid = load_data()
    
    candidates = []
    for y in range(len(grid)):
        candidates += [(0, y, Direction.EAST), (len(grid[0])-1, y, Direction.WEST)]
    for x in range(len(grid[0])):
        candidates += [(x, 0, Direction.SOUTH), (x, len(grid)-1, Direction.NORTH)]

    max_energized = 0
    for candidate in candidates:
        energized = set([candidate[:2]])
        seen = set([candidate])
        todo = [candidate]
        while len(todo) > 0:
            cur = todo.pop()
            next_positions = step(grid, cur[0], cur[1], cur[2])
            for next_position in next_positions:
                if next_position not in seen:
                    seen.add(next_position)
                    todo += [next_position]
                    energized.add(next_position[:2])
        max_energized = max(max_energized, len(energized))

    print(f"{max_energized}")
    