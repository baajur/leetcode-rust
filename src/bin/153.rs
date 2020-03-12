struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0; // is is correct?
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let last = *nums.last().unwrap();
        let first = nums[0];

        if last > first {
            // in this case nums are sorted, not rotated
            return nums[0];
        }

        let pivot = {
            let mut ok = nums.len();
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid] <= last {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        nums[pivot]
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(Solution::find_min(vec![]), 0);
        assert_eq!(Solution::find_min(vec![2]), 2);
        assert_eq!(Solution::find_min(vec![3, 4, 5, 6, 1]), 1);
    }
}
