struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() <= 1 {
            return true;
        }
        let mut intervals = intervals;
        use std::cmp::Ordering;
        intervals.sort_by(|a, b| match a[0].cmp(&b[0]) {
            Ordering::Equal => a[1].cmp(&b[1]),
            x => x,
        });

        for i in 0..(intervals.len() - 1) {
            let end_time = intervals[i][1];
            let start_time = intervals[i + 1][0];
            if end_time > start_time {
                return false;
            }
        }

        true
    }
}

fn main() {
    Solution::can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]);
    Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4], vec![6, 9], vec![7, 8]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_attend_meetings() {
        assert!(!Solution::can_attend_meetings(vec![
            vec![0, 30],
            vec![5, 10],
            vec![15, 20]
        ]));
        assert!(Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
        assert!(Solution::can_attend_meetings(vec![vec![7, 10], vec![1, 7]]));
        assert!(Solution::can_attend_meetings(vec![]));
        assert!(Solution::can_attend_meetings(vec![vec![1, 20]]));
        assert!(!Solution::can_attend_meetings(vec![
            vec![7, 10],
            vec![7, 8]
        ]));
    }
}
