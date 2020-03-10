struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let first = nums[0];
        let last = nums[nums.len() - 1];

        let idx = if first <= last {
            // in case no rotation
            let mut ok = 0;
            let mut ng = nums.len();
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid] > target {
                    ng = mid;
                } else {
                    ok = mid;
                }
            }
            ok
        } else if target >= nums[0] {
            // target value is in the left section
            let mut ok = 0;
            let mut ng = nums.len();
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid] <= last || nums[mid] > target {
                    ng = mid;
                } else {
                    ok = mid;
                }
            }
            ok
        } else {
            // it's in the right section
            let mut ok = nums.len() - 1;
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid] >= first || nums[mid] < target {
                    ng = mid;
                } else {
                    ok = mid;
                }
            }
            ok
        };

        if nums[idx] == target {
            idx as i32
        } else {
            -1
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
    fn test_search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 5), 1);
        assert_eq!(Solution::search(vec![], 5), -1);
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 2), 1);
        assert_eq!(Solution::search(vec![1], 1), 0);
    }
}
