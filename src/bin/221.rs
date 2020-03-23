struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let h = matrix.len();
        let w = matrix[0].len();
        let mut acc_sum = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                acc_sum[i + 1][j + 1] = acc_sum[i][j + 1] + acc_sum[i + 1][j] - acc_sum[i][j]
                    + (matrix[i][j] as i32 - '0' as i32);
            }
        }

        let mut max_area = 0;
        for l in 0..std::cmp::min(h, w) {
            // edge length is l
            for i in 0..(h - l) {
                for j in 0..(w - l) {
                    let area = acc_sum[i + l + 1][j + l + 1]
                        - acc_sum[i + l + 1][j]
                        - acc_sum[i][j + l + 1]
                        + acc_sum[i][j];
                    if area == ((l + 1) * (l + 1)) as i32 {
                        max_area = std::cmp::max(max_area, area);
                    }
                }
            }
        }
        max_area
    }
}

fn main() {
    dbg!(Solution::maximal_square(vec![
        vec!['1', '0', '1'],
        vec!['0', '1', '1'],
        vec!['0', '1', '1'],
    ]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_square() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1'],
                vec!['0', '1', '1'],
                vec!['0', '1', '1'],
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '1', '1', '0', '0'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '0'],
            ]),
            9
        );
        assert_eq!(
            Solution::maximal_square(vec![vec!['1', '0', '1', '0', '0'],]),
            1
        );
        assert_eq!(Solution::maximal_square(vec![]), 0);
        assert_eq!(Solution::maximal_square(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_square(vec![vec!['0']]), 0);
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '1', '1', '1'],
            ]),
            9
        );
    }
}
