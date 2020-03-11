struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 0;
        }
        if obstacle_grid[0].is_empty() {
            return 0;
        }
        if obstacle_grid[0][0] == 1 || obstacle_grid.last().unwrap().last().unwrap() == &1 {
            return 0;
        }

        let height = obstacle_grid.len();
        let width = obstacle_grid[0].len();
        let mut dp = vec![vec![0; width]; height];
        dp[0][0] = 1;

        let is_obstacle = |y: usize, x: usize| obstacle_grid[y][x] == 1;

        for h in 0..height {
            for w in 0..width {
                if w + 1 < width && !is_obstacle(h, w + 1) {
                    // go right
                    dp[h][w + 1] += dp[h][w];
                }
                if h + 1 < height && !is_obstacle(h + 1, w) {
                    // go down
                    dp[h + 1][w] += dp[h][w];
                }
            }
        }

        dp[height - 1][width - 1]
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );

        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0],]),
            1
        );

        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![], vec![],]),
            0
        );

        assert_eq!(Solution::unique_paths_with_obstacles(vec![]), 0);

        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
    }
}
