struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }
        // transpose
        for h in 0..(matrix.len() + 1 / 2) {
            for w in h..matrix[0].len() {
                let tmp = matrix[w][h];
                matrix[w][h] = matrix[h][w];
                matrix[h][w] = tmp;
            }
        }

        for h in 0..matrix.len() {
            matrix[h].reverse();
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);

    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
}

// TODO: write tests
