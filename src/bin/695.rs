struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        use std::cmp::max;
        use std::collections::HashSet;
        let mut visited = HashSet::new();
        let mut max_area = 0;
        for h in 0..grid.len() {
            for w in 0..grid[0].len() {
                if grid[h][w] == 0 || visited.contains(&(h, w)) {
                    continue;
                }
                max_area = max(max_area, dfs(&mut visited, &grid, h, w));
            }
        }
        max_area
    }
}

fn dfs(
    visited: &mut std::collections::HashSet<(usize, usize)>,
    grid: &Vec<Vec<i32>>,
    y: usize,
    x: usize,
) -> i32 {
    match grid[y][x] {
        0 => 0,
        1 => {
            visited.insert((y, x));

            let mut count = 1;
            // up
            if y >= 1 && !visited.contains(&(y - 1, x)) {
                count += dfs(visited, grid, y - 1, x);
            }
            // down
            if y + 1 <= grid.len() - 1 && !visited.contains(&(y + 1, x)) {
                count += dfs(visited, grid, y + 1, x);
            }
            // left
            if x >= 1 && !visited.contains(&(y, x - 1)) {
                count += dfs(visited, grid, y, x - 1);
            }
            // right
            if x + 1 <= grid[0].len() - 1 && !visited.contains(&(y, x + 1)) {
                count += dfs(visited, grid, y, x + 1);
            }
            count
        }
        _ => unreachable!(),
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_of_island() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ]),
            6
        );

        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0],]),
            0
        );

        assert_eq!(Solution::max_area_of_island(vec![vec![]]), 0);

        assert_eq!(Solution::max_area_of_island(vec![]), 0);
    }
}
