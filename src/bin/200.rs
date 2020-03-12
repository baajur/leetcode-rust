struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut marker = grid.clone();
        let mut cnt = 0;

        for h in 0..grid.len() {
            for w in 0..grid[0].len() {
                if grid[h][w] == '0' || marker[h][w] == 'x' {
                    continue;
                }
                cnt += 1;
                Solution::mark_island(&mut marker, h, w);
            }
        }

        cnt
    }

    fn mark_island(marker: &mut Vec<Vec<char>>, h: usize, w: usize) {
        match marker[h][w] {
            '0' | 'x' => (),
            _ => {
                marker[h][w] = 'x';
                let height = marker.len();
                let width = marker[0].len();
                if h >= 1 {
                    Solution::mark_island(marker, h - 1, w);
                }
                if h + 1 < height {
                    Solution::mark_island(marker, h + 1, w);
                }
                if w >= 1 {
                    Solution::mark_island(marker, h, w - 1);
                }
                if w + 1 < width {
                    Solution::mark_island(marker, h, w + 1);
                }
            }
        }
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ]),
            3
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '1', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '1', '0', '1', '1'],
                vec!['0', '1', '0', '1', '1'],
            ]),
            3
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '1', '0', '0'],
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![vec!['1', '1', '1', '1', '1'],]),
            1
        );
        assert_eq!(Solution::num_islands(vec![]), 0);
    }
}
