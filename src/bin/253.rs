struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        if intervals.len() == 1 {
            return 1;
        }
        use std::collections::BTreeMap;
        let mut m: BTreeMap<i32, (i32, i32)> = BTreeMap::new();
        for i in 0..intervals.len() {
            let start_time = intervals[i][0];
            let end_time = intervals[i][1];
            {
                let start_entry = m.entry(start_time).or_insert((0, 0));
                start_entry.0 += 1;
            }
            {
                let end_entry = m.entry(end_time).or_insert((0, 0));
                end_entry.1 += 1;
            }
        }

        let mut ret = -1 as i32;
        let mut cur = 0i32;

        for (key, (start, end)) in m.into_iter() {
            cur += start - end;
            ret = std::cmp::max(ret, cur);
        }

        ret
    }
}

fn main() {
    Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_meeting_rooms() {
        assert_eq!(Solution::min_meeting_rooms(vec![]), 0);
        assert_eq!(Solution::min_meeting_rooms(vec![vec![1, 20]]), 1);
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            2
        );
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![0, 30], vec![9, 12], vec![5, 10], vec![15, 20]]),
            3
        );
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]),
            1
        );
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![7, 10], vec![2, 7], vec![1, 2]]),
            1
        );
    }
}
