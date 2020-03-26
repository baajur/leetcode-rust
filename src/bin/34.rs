struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let n = nums.len() as i32;
        let upper_bound = {
            let mut ok = -1;
            let mut ng = n;
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid as usize] > target {
                    ng = mid;
                } else {
                    ok = mid;
                }
            }
            ok
        };
        let lower_bound = {
            let mut ok = n;
            let mut ng = -1;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid as usize] >= target {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        if upper_bound - lower_bound >= 0 {
            vec![lower_bound, upper_bound]
        } else {
            vec![-1, -1]
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
    fn test_search_range() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![1, 2, 3], 1), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![1, 2, 3], 3), vec![2, 2]);
        assert_eq!(Solution::search_range(vec![1, 1, 3], 1), vec![0, 1]);
        assert_eq!(Solution::search_range(vec![2, 3, 4], 1), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 1), vec![-1, -1]);
    }
}
