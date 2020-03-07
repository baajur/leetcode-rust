struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(&idx) => return vec![idx, i as i32],
                None => {
                    map.insert(nums[i], i as i32);
                }
            }
        }
        unreachable!();
    }

    pub fn binary_search_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let entry = map.entry(nums[i]).or_insert(vec![]);
            entry.push(i as i32);
        }
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            let j = {
                let mut ok = 0;
                let mut ng = nums.len();
                while ng - ok > 1 {
                    let mid = (ok + ng) / 2;
                    if nums[i] + nums[mid] <= target {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                ok
            };
            if nums[i] + nums[j] == target {
                if nums[i] == nums[j] {
                    return vec![map.get(&nums[i]).unwrap()[0], map.get(&nums[j]).unwrap()[1]];
                }
                return vec![map.get(&nums[i]).unwrap()[0], map.get(&nums[j]).unwrap()[0]];
            }
        }
        vec![42]
    }

    pub fn brute_force_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![42]
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_binary_search_two_sum() {
        assert_eq!(
            Solution::binary_search_two_sum(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
    }

    #[test]
    fn test_brute_force_two_sum() {
        assert_eq!(
            Solution::brute_force_two_sum(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
    }
}
