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