use std::collections::VecDeque;
use std::ops::Neg;

#[derive(PartialEq, Clone, Copy)]
pub enum Part {
    First,
    Second
}

#[macro_export]
macro_rules! debug {
 ($e:expr) => {
     if env::var("AOC_DEBUG").is_ok() {
        println!("{:?}", $e);
    }
 };
}

pub fn get_grid_neigh_indexes(pos: (usize, usize), grid_size:(usize, usize)) -> Vec<(usize, usize)> {
    let mut i_diffs: Vec<(usize, usize)> = vec![];
    if pos.0 > 0 {
        i_diffs.push((pos.0 - 1, pos.1));
        if pos.1 > 0 { i_diffs.push((pos.0 - 1, pos.1 - 1)); }
        if pos.1 < grid_size.1 - 1 { i_diffs.push((pos.0 - 1, pos.1 + 1)); }
    }
    if pos.1 > 0 { i_diffs.push((pos.0, pos.1 - 1)); }
    if pos.1 < grid_size.1 - 1 { i_diffs.push((pos.0, pos.1 + 1)); }
    if pos.0 < grid_size.0 - 1 {
        i_diffs.push((pos.0 + 1, pos.1));
        if pos.1 > 0 { i_diffs.push((pos.0 + 1, pos.1 - 1)); }
        if pos.1 < grid_size.1 - 1 { i_diffs.push((pos.0 + 1, pos.1 + 1)); }
    }
    i_diffs
}


struct Node {
    pos: (usize, usize),
    distance: u32
}

pub fn find_shortest_path<T>(start: (usize, usize), end: (usize, usize), grid: &Vec<Vec<T>>) -> u32 {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    grid[start.0][start.1] = true;

    let mut q: VecDeque<Node> = VecDeque::from(vec![Node { pos: start, distance: 0 }]);

    loop {
        let Node { pos, distance: dist } = match q.pop_front() {
            None => { break; }
            Some(front) => { front }
        };

        if pos == end { return dist; }

        // up
        if pos.0 > 0 && !grid[pos.0 - 1][pos.1] {
            q.push_back(Node { pos: (pos.0 - 1, pos.1), distance: dist + 1 });
            grid[pos.0 - 1][pos.1] = true;
        }

        // down
        if pos.0 < grid.len() - 1 && !grid[pos.0 + 1][pos.1] {
            q.push_back(Node { pos: (pos.0 + 1, pos.1), distance: dist + 1 });
            grid[pos.0 + 1][pos.1] = true;
        }

        // left
        if pos.1 > 0 && !grid[pos.0][pos.1 - 1] {
            q.push_back(Node { pos: (pos.0, pos.1 - 1), distance: dist + 1 });
            grid[pos.0][pos.1 - 1] = true;
        }

        // right
        if pos.1 < grid[0].len() - 1 && !grid[pos.0][pos.1 + 1] {
            q.push_back(Node { pos: (pos.0, pos.1 + 1), distance: dist + 1 });
            grid[pos.0][pos.1 + 1] = true;
        }
    }

    0
}

pub fn usize_safe_sub(left: usize, right: usize) -> i8 {
    if left > right { i8::try_from(left - right ).expect("Failed to interpret usize - usize result as i8!") }
    else { i8::try_from(right-left).expect("Failed to interpret usize - usize result as i8!").neg() }
}

#[test]
fn test_get_grid_neigh_indexes() {
    let grid: Vec<Vec<bool>> = vec![vec![false, false, false], vec![false, false, false], vec![false, false, false]];
    let grid_size = (grid.len(), grid[0].len());
    assert_eq!(get_grid_neigh_indexes((0, 0), grid_size), vec![(0, 1), (1, 0), (1, 1)]);
    assert_eq!(get_grid_neigh_indexes((1, 1), grid_size).len(), 8);
    assert_eq!(get_grid_neigh_indexes((2, 1), grid_size).len(), 5);
}

#[test]
fn test_usize_safe_sub() {
    assert_eq!(usize_safe_sub(2, 1), 1);
    assert_eq!(usize_safe_sub(1, 2), -1);
}

#[test]
fn test_find_shortest_path() {
    let grid: Vec<Vec<u8>> = vec![vec![0;4]; 4];
    assert_eq!(find_shortest_path((0, 1), (3, 3), &grid), 5);
}