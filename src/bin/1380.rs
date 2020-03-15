struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }
        let height = matrix.len();
        let width = matrix[0].len();
        let mut ret = Vec::new();
        for h in 0..height {
            let min_in_row = matrix[h].iter().enumerate().min_by_key(|x| x.1).unwrap();
            let mut ok = true;
            for hh in 0..height {
                if matrix[hh][min_in_row.0] > *min_in_row.1 {
                    ok = false;
                    break;
                }
            }
            if ok {
                ret.push(*min_in_row.1);
            }
        }
        ret
    }
}

fn main() {
    dbg!(Solution::lucky_numbers(vec![
        vec![3, 7, 8],
        vec![9, 11, 13],
        vec![15, 16, 17]
    ]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2],]),
            vec![7]
        );
        assert_eq!(Solution::lucky_numbers(vec![]), vec![]);
        assert_eq!(Solution::lucky_numbers(vec![vec![]]), vec![]);
    }
}
