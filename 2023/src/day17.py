import astar
import utils

def load_data():
    with open("data/day17.txt", "r") as file:
        data = file.read().splitlines()
    
    grid = []
    for line in data:
        row = [int(c) for c in line]
        grid += [row]
    return grid

def part1():
    grid = load_data()
    
    start = (0, 0, utils.Direction.EAST, 0)
    goal = (len(grid)-1, len(grid[0])-1, utils.Direction.EAST, 0)
    
    solution = Solver(grid, part=1).astar(start, goal)
    heat_loss = sum([grid[x[1]][x[0]] for x in solution]) - grid[start[1]][start[0]]

    print(f"{heat_loss}")

class Solver(astar.AStar):
    def __init__(self, grid, part=1):
        self.grid = grid
        self.part = part

    def neighbors(self, node):
        result = []
        # Straight
        pos = utils.step(node[:2], node[2])
        if self.bound_check(pos) and (self.part == 1 and node[3] < 3 or self.part == 2 and node[3] < 10):
            result += [(pos[0], pos[1], node[2], node[3] + 1)]

        # Right
        dir = utils.rotate_cw(node[2])
        pos = utils.step(node[:2], dir)
        if self.bound_check(pos) and (self.part == 1 or node[3] >= 4):
            result += [(pos[0], pos[1], dir, 1)]

        # Left
        dir = utils.rotate_ccw(node[2])
        pos = utils.step(node[:2], dir)
        if self.bound_check(pos) and (self.part == 1 or node[3] >= 4):
            result += [(pos[0], pos[1], dir, 1)]

        return result
    
    def distance_between(self, n1, n2):
        return self.grid[n2[1]][n2[0]]

    def heuristic_cost_estimate(self, current, goal):
        return abs(goal[0] - current[0]) + abs(goal[1] - current[1])

    def is_goal_reached(self, current, goal):
        return current[0] == goal[0] and current[1] == goal[1] and (self.part == 1 or current[3] >= 4)
    
    def bound_check(self, pos):
        return pos[0] >= 0 and pos[0] < len(self.grid[0]) and pos[1] >= 0 and pos[1] < len(self.grid)

def part2():
    grid = load_data()
    
    start = (0, 0, utils.Direction.EAST, 0)
    goal = (len(grid)-1, len(grid[0])-1, utils.Direction.EAST, 0)
    
    solution = Solver(grid, part=2).astar(start, goal)
    heat_loss = sum([grid[x[1]][x[0]] for x in solution]) - grid[start[1]][start[0]]

    print(f"{heat_loss}")
    
    