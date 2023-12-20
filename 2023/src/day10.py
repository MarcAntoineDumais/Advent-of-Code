def load_data():
    file = open("data/day10.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def part1():
    grid = load_data()
    s = None
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                s = {"pos": (x, y), "distance": 0, "tile": "S"}
    
    found = set([s["pos"]])
    todo = [s]
    max_distance = 0
    while len(todo) > 0:
        cur = todo.pop()
        pos = cur["pos"]
        max_distance = max(max_distance, cur["distance"])
        
        # Up
        next_pos = (pos[0], pos[1]-1)
        if cur["tile"] in "SJL|" and pos[1] > 0 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SF7|":
            found.add(next_pos)
            todo = [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}] + todo
        # Down
        next_pos = (pos[0], pos[1]+1)
        if cur["tile"] in "SF7|" and pos[1] < len(grid) - 1 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SJL|":
            found.add(next_pos)
            todo = [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}] + todo
        # Left
        next_pos = (pos[0]-1, pos[1])
        if cur["tile"] in "SJ7-" and pos[0] > 0 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SFL-":
            found.add(next_pos)
            todo = [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}] + todo
        # Right
        next_pos = (pos[0]+1, pos[1])
        if cur["tile"] in "SFL-" and pos[0] < len(grid[0]) - 1 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SJ7-":
            found.add(next_pos)
            todo = [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}] + todo

    print(f"{max_distance}")

def part2():
    grid = load_data()
    s = None
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == "S":
                s = {"pos": (x, y), "distance": 0, "tile": "S"}
    
    found = set([s["pos"]])
    todo = [s]
    loop = []
    while len(todo) > 0:
        cur = todo.pop()
        loop += [cur]
        pos = cur["pos"]
        
        # Up = 0
        next_pos = (pos[0], pos[1]-1)
        if cur["tile"] in "SJL|" and pos[1] > 0 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SF7|":
            found.add(next_pos)
            todo += [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}]
        # Right = 1
        next_pos = (pos[0]+1, pos[1])
        if cur["tile"] in "SFL-" and pos[0] < len(grid[0]) - 1 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SJ7-":
            found.add(next_pos)
            todo += [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}]
        # Down = 2
        next_pos = (pos[0], pos[1]+1)
        if cur["tile"] in "SF7|" and pos[1] < len(grid) - 1 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SJL|":
            found.add(next_pos)
            todo += [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}]
        # Left = 3
        next_pos = (pos[0]-1, pos[1])
        if cur["tile"] in "SJ7-" and pos[0] > 0 and next_pos not in found and grid[next_pos[1]][next_pos[0]] in "SFL-":
            found.add(next_pos)
            todo += [{"pos": next_pos, "distance": cur["distance"] + 1, "tile": grid[next_pos[1]][next_pos[0]]}]

    # Make sure the loop goes clockwise
    cur = s
    direction = None
    winding = 0
    for node in loop[1:]:
        prev = cur
        cur = node
        prev_direction = direction
        direction = get_direction(prev, cur)
        if prev_direction is not None:
            winding += get_rotation(prev_direction, direction)

    if winding < 0:
        loop.reverse()

    # Count enclosed tiles
    explored = set([x["pos"] for x in loop])
    todo = []
    prev = loop[0]

    for cur in loop[1:]:
        direction = get_direction(prev, cur)
        candidates = [prev["pos"], cur["pos"]]
        for candidate in candidates:
            if direction == 0:
                candidate = (candidate[0] + 1, candidate[1])
            elif direction == 1:
                candidate = (candidate[0], candidate[1] + 1)
            elif direction == 2:
                candidate = (candidate[0] - 1, candidate[1])
            elif direction == 3:
                candidate = (candidate[0], candidate[1] - 1)
        
            if candidate not in explored:
                explored.add(candidate)
                todo += [candidate]

        prev = cur

    enclosed = set()
    while len(todo) > 0:
        pos = todo.pop()
        enclosed.add(pos)
        for candidate in [(pos[0], pos[1] - 1), (pos[0] + 1, pos[1]), (pos[0], pos[1] + 1), (pos[0] - 1, pos[1])]:
            if candidate not in explored:
                todo += [candidate]
                explored.add(candidate)
    print(f"{len(enclosed)}")

    # Write simplified loop to file for debugging
    # loop_set = set([x["pos"] for x in loop])
    # with open("data/day10_clean.txt", "w") as file:
    #     for y in range(len(grid)):
    #         line = ""
    #         for x in range(len(grid[y])):
    #             if (x, y) in enclosed:
    #                 line += ","
    #             elif (x, y) in loop_set:
    #                 line += grid[y][x]
    #             else:
    #                 line += "."
    #         file.write(line + "\n")


def get_direction(prev, cur):
    if cur["pos"][1] < prev["pos"][1]:
        return 0
    if cur["pos"][0] > prev["pos"][0]:
        return 1
    if cur["pos"][1] > prev["pos"][1]:
        return 2
    return 3

def get_rotation(prev_dir, cur_dir):
    if prev_dir == cur_dir:
        return 0
    if cur_dir == (prev_dir + 1) % 4:
        return 1
    if cur_dir == (prev_dir - 1) % 4:
        return -1
    
    print("invalid rotation")
    exit()