struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let mut h = 0;
        let mut w = 0;
        let max_h = grid.len();
        let max_w = grid[0].len();
        use std::collections::{HashSet, VecDeque};
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back((h, w));

        while !q.is_empty() {
            let (h, w) = q.pop_front().unwrap();
            if h == max_h - 1 && w == max_w - 1 {
                return true;
            }
            if visited.contains(&(h, w)) {
                continue;
            }
            visited.insert((h, w));

            match grid[h][w] {
                1 => {
                    // go right or left
                    // right
                    if w + 1 < max_w && right_con(&grid, h, w + 1) {
                        q.push_back((h, w + 1));
                    }
                    // left
                    if w > 0 && left_con(&grid, h, w - 1) {
                        q.push_back((h, w - 1));
                    }
                }
                2 => {
                    // go down or up
                    // down
                    if h + 1 < max_h && down_con(&grid, h + 1, w) {
                        q.push_back((h + 1, w));
                    }
                    // up
                    if h > 0 && up_con(&grid, h - 1, w) {
                        q.push_back((h - 1, w));
                    }
                }
                3 => {
                    // go left or down
                    // down
                    if h + 1 < max_h && down_con(&grid, h + 1, w) {
                        q.push_back((h + 1, w));
                    }
                    // left
                    if w > 0 && left_con(&grid, h, w - 1) {
                        q.push_back((h, w - 1));
                    }
                }
                4 => {
                    // go right or down
                    // right
                    if w + 1 < max_w && right_con(&grid, h, w + 1) {
                        q.push_back((h, w + 1));
                    }
                    // down
                    if h + 1 < max_h && down_con(&grid, h + 1, w) {
                        q.push_back((h + 1, w));
                    }
                }
                5 => {
                    // go up or left
                    // up
                    if h > 0 && up_con(&grid, h - 1, w) {
                        q.push_back((h - 1, w));
                    }
                    // left
                    if w > 0 && left_con(&grid, h, w - 1) {
                        q.push_back((h, w - 1));
                    }
                }
                6 => {
                    // go up or right
                    // up
                    if h > 0 && up_con(&grid, h - 1, w) {
                        q.push_back((h - 1, w));
                    }
                    // right
                    if w + 1 < max_w && right_con(&grid, h, w + 1) {
                        q.push_back((h, w + 1));
                    }
                }
                _ => unreachable!(),
            }
        }

        false
    }
}

fn right_con(grid: &Vec<Vec<i32>>, h: usize, w: usize) -> bool {
    grid[h][w] == 1 || grid[h][w] == 3 || grid[h][w] == 5
}

fn left_con(grid: &Vec<Vec<i32>>, h: usize, w: usize) -> bool {
    grid[h][w] == 1 || grid[h][w] == 4 || grid[h][w] == 6
}

fn up_con(grid: &Vec<Vec<i32>>, h: usize, w: usize) -> bool {
    grid[h][w] == 2 || grid[h][w] == 3 || grid[h][w] == 4
}

fn down_con(grid: &Vec<Vec<i32>>, h: usize, w: usize) -> bool {
    grid[h][w] == 2 || grid[h][w] == 5 || grid[h][w] == 6
}

fn main() {
    assert!(Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]));
    assert!(Solution::has_valid_path(vec![vec![4, 1], vec![6, 1]]));
    assert!(!Solution::has_valid_path(vec![
        vec![1, 2, 1],
        vec![1, 2, 1]
    ]));
    assert!(!Solution::has_valid_path(vec![vec![1, 1, 2]]));
    assert!(Solution::has_valid_path(vec![vec![1, 1, 1, 1, 1, 1, 3]]));
    assert!(Solution::has_valid_path(vec![
        vec![2],
        vec![2],
        vec![2],
        vec![2],
        vec![2],
        vec![2],
        vec![6],
    ]));
}
