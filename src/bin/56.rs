struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        use std::cmp::Ordering;
        let mut intervals = intervals;
        intervals.sort_by(|a, b| match a[0].cmp(&b[0]) {
            Ordering::Equal => a[1].cmp(&b[1]),
            x => x,
        });
        let mut ret = Vec::new();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for i in 0..intervals.len() {
            if i == intervals.len() - 1 {
                ret.push(vec![start, std::cmp::max(end, intervals[i][1])]);
            } else {
                if end < intervals[i + 1][0] {
                    ret.push(vec![start, end]);
                    start = intervals[i + 1][0];
                    end = intervals[i + 1][1];
                } else if end >= intervals[i + 1][1] {
                    continue;
                } else {
                    end = intervals[i + 1][1];
                }
            }
        }
        ret
    }
}

fn main() {
    Solution::merge(vec![vec![1, 3], vec![1, 2], vec![8, 10], vec![15, 18]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![6, 10], vec![15, 18]]),
            vec![vec![1, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![-1, 0], vec![0, 3], vec![6, 10], vec![8, 18]]),
            vec![vec![-1, 3], vec![6, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 3]]),
            vec![vec![1, 4]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 5], vec![2, 3], vec![3, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );

        let expected: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::merge(vec![]), expected);
    }
}
