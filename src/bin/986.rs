struct Solution;

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut intersections = Vec::new();
        let mut a_idx = 0;
        let mut b_idx = 0;
        while a_idx < a.len() && b_idx < b.len() {
            let a_begin = a[a_idx][0];
            let b_begin = b[b_idx][0];
            let a_end = a[a_idx][1];
            let b_end = b[b_idx][1];

            if a_begin <= b_begin {
                if b_end <= a_end {
                    intersections.push(vec![b_begin, b_end]);
                    b_idx += 1;
                } else if b_begin <= a_end && a_end <= b_end {
                    intersections.push(vec![b_begin, a_end]);
                    a_idx += 1;
                } else {
                    a_idx += 1;
                }
            } else {
                if a_end <= b_end {
                    intersections.push(vec![a_begin, a_end]);
                    a_idx += 1;
                } else if a_begin <= b_end && b_end <= a_end {
                    intersections.push(vec![a_begin, b_end]);
                    b_idx += 1;
                } else {
                    b_idx += 1;
                }
            }
        }
        intersections
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_intersection() {
        assert_eq!(
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25],],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26],]
            ),
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25],
            ]
        );
        assert_eq!(
            Solution::interval_intersection(
                vec![vec![0, 20]],
                vec![vec![2, 5], vec![6, 10], vec![18, 21]]
            ),
            vec![vec![2, 5], vec![6, 10], vec![18, 20]]
        );
        assert!(Solution::interval_intersection(vec![vec![0, 5]], vec![vec![6, 10]]).is_empty());
        assert!(Solution::interval_intersection(vec![], vec![]).is_empty());
    }
}
