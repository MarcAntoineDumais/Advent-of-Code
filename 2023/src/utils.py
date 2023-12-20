from enum import Enum

class Direction(Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

def rotate_cw(dir):
    match dir:
        case Direction.NORTH:
            return Direction.EAST
        case Direction.EAST:
            return Direction.SOUTH
        case Direction.SOUTH:
            return Direction.WEST
        case Direction.WEST:
            return Direction.NORTH
    return dir

def rotate_ccw(dir):
    match dir:
        case Direction.NORTH:
            return Direction.WEST
        case Direction.EAST:
            return Direction.NORTH
        case Direction.SOUTH:
            return Direction.EAST
        case Direction.WEST:
            return Direction.SOUTH
    return dir

def step_increment(dir):
    match dir:
        case Direction.NORTH:
            return (0, -1)
        case Direction.EAST:
            return (1, 0)
        case Direction.SOUTH:
            return (0, 1)
        case Direction.WEST:
            return (-1, 0)
    return (0, 0)

def step(pos, dir):
    increment = step_increment(dir)
    return (pos[0] + increment[0], pos[1] + increment[1])