struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let index = {
            let mut ok = 0;
            let mut ng = nums.len();
            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if nums[mid] <= target {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok + (nums[ok] < target) as usize
        };

        index as i32
    }
}

fn main() {
    Solution::search_insert(vec![1, 3, 5, 6], 5);
    Solution::search_insert(vec![1, 3, 5, 6], 2);
    Solution::search_insert(vec![1, 3, 5, 6], 7);
    Solution::search_insert(vec![1, 3, 5, 6], 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let test_cases = [
            (vec![1, 3, 5, 6], 5, 2),
            (vec![1, 3, 5, 6], 2, 1),
            (vec![1, 3, 5, 6], 7, 4),
            (vec![1, 3, 5, 6], 0, 0),
            (vec![1, 3, 5, 8], 6, 3),
            (vec![1, 3, 5, 9, 13], 13, 4),
        ];

        for test in test_cases.iter() {
            assert_eq!(Solution::search_insert(test.0.clone(), test.1), test.2);
        }
    }
}
