use std::fs;

use crate::utils::{get_2d_index, get_coords_from_index};

pub fn run() {
    let data = fs::read_to_string("data/day8.txt").unwrap();
    let lines: Vec<&str> = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let height = lines.len();
    let width = lines[0].chars().collect::<Vec<char>>().len();

    let mut trees: Vec<Tree> = Vec::new();
    for line in lines.iter() {
        let tree_row = line.chars().map(|c| Tree {
            height: c.to_digit(10).unwrap() as i32,
            visible: false,
        });
        for tree in tree_row {
            trees.push(tree);
        }
    }

    for col in 0..width {
        let mut highest_tree = -1;
        for row in 0..height {
            let i = get_2d_index(row, col, width);
            if trees[i].height > highest_tree {
                trees[i].visible = true;
                highest_tree = trees[i].height;
            }
        }

        highest_tree = -1;
        for row in (0..height).rev() {
            let i = get_2d_index(row, col, width);
            if trees[i].height > highest_tree {
                trees[i].visible = true;
                highest_tree = trees[i].height;
            }
        }
    }

    for row in 0..height {
        let mut highest_tree = -1;
        for col in 0..width {
            let i = get_2d_index(row, col, width);
            if trees[i].height > highest_tree {
                trees[i].visible = true;
                highest_tree = trees[i].height;
            }
        }

        highest_tree = -1;
        for col in (0..width).rev() {
            let i = get_2d_index(row, col, width);
            if trees[i].height > highest_tree {
                trees[i].visible = true;
                highest_tree = trees[i].height;
            }
        }
    }

    let visible_trees = trees.iter().filter(|t| t.visible).count();
    println!("{visible_trees}");

    // Part 2
    let highest_score = trees
        .iter()
        .enumerate()
        .map(|(i, tree)| scenic_score(i, tree, &trees, width, height))
        .max()
        .unwrap();
    println!("{highest_score}");
}

struct Tree {
    height: i32,
    visible: bool,
}

fn scenic_score(i: usize, tree: &Tree, trees: &Vec<Tree>, width: usize, height: usize) -> i32 {
    let (row, col) = get_coords_from_index(i, width);
    let mut score = 1;

    let mut cur_score = 0;
    for col in (col + 1)..width {
        cur_score += 1;
        if trees[get_2d_index(row, col, width)].height >= tree.height {
            break;
        }
    }
    score *= cur_score;

    cur_score = 0;
    for col in (0..col).rev() {
        cur_score += 1;
        if trees[get_2d_index(row, col, width)].height >= tree.height {
            break;
        }
    }
    score *= cur_score;

    cur_score = 0;
    for row in (row + 1)..height {
        cur_score += 1;
        if trees[get_2d_index(row, col, width)].height >= tree.height {
            break;
        }
    }
    score *= cur_score;

    cur_score = 0;
    for row in (0..row).rev() {
        cur_score += 1;
        if trees[get_2d_index(row, col, width)].height >= tree.height {
            break;
        }
    }
    score *= cur_score;

    score
}
