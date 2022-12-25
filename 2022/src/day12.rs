use std::fs;

use pathfinding::prelude::astar;

use crate::utils::{get_2d_index, get_coords_from_index};

pub fn run() {
    let data = fs::read_to_string("data/day12.txt").unwrap();
    let lines: Vec<&str> = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let height = lines.len();
    let width = lines[0].chars().collect::<Vec<char>>().len();

    let mut height_map: Vec<i32> = Vec::new();
    let mut start = Cell { x: 0, y: 0 };
    let mut goal = Cell { x: 0, y: 0 };
    for (y, line) in lines.iter().enumerate() {
        let row = line.chars().enumerate().map(|(x, c)| match c {
            'S' => {
                start.x = x;
                start.y = y;
                1
            }
            'E' => {
                goal.x = x;
                goal.y = y;
                26
            }
            c => c as i32 - 'a' as i32 + 1,
        });
        for element in row {
            height_map.push(element);
        }
    }

    let starts: Vec<Cell> = height_map
        .iter()
        .enumerate()
        .filter(|(_, h)| **h == 1)
        .map(|(i, _)| {
            let coords = get_coords_from_index(i, width);
            Cell {
                x: coords.1,
                y: coords.0,
            }
        })
        .collect();

    let mut min_cost = i32::MAX;
    for start in starts {
        let path = astar(
            &start,
            successors(width, height, height_map.clone()),
            |cell| (goal.x as i32 - cell.x as i32).abs() + (goal.y as i32 - cell.y as i32).abs(),
            |cell| *cell == goal,
        );
        if let Some(x) = path {
            if x.1 < min_cost {
                min_cost = x.1;
            }
        }
    }

    println!("{min_cost}");
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cell {
    x: usize,
    y: usize,
}

fn successors(
    width: usize,
    height: usize,
    height_map: Vec<i32>,
) -> Box<dyn Fn(&Cell) -> Vec<(Cell, i32)>> {
    Box::new(move |cell: &Cell| {
        let cell_height = height_map[get_2d_index(cell.y, cell.x, width)];
        let mut succ = Vec::new();
        if cell.x < width - 1
            && cell_height + 1 >= height_map[get_2d_index(cell.y, cell.x + 1, width)]
        {
            succ.push((
                Cell {
                    x: cell.x + 1,
                    y: cell.y,
                },
                1,
            ));
        }
        if cell.x > 0 && cell_height + 1 >= height_map[get_2d_index(cell.y, cell.x - 1, width)] {
            succ.push((
                Cell {
                    x: cell.x - 1,
                    y: cell.y,
                },
                1,
            ));
        }
        if cell.y < height - 1
            && cell_height + 1 >= height_map[get_2d_index(cell.y + 1, cell.x, width)]
        {
            succ.push((
                Cell {
                    x: cell.x,
                    y: cell.y + 1,
                },
                1,
            ));
        }
        if cell.y > 0 && cell_height + 1 >= height_map[get_2d_index(cell.y - 1, cell.x, width)] {
            succ.push((
                Cell {
                    x: cell.x,
                    y: cell.y - 1,
                },
                1,
            ));
        }
        succ
    })
}
