struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        if nums.len() == 1 {
            return vec![vec![nums[0]]];
        }

        let mut nums = nums;
        nums.sort_unstable();

        let mut ret = Vec::new();
        ret.push(nums.clone());
        while Solution::next_permutation(&mut nums) {
            ret.push(nums.clone());
        }

        ret
    }

    fn next_permutation(nums: &mut Vec<i32>) -> bool {
        let mut index = None;
        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] < nums[i + 1] {
                index = Some(i);
                break;
            }
        }

        if index.is_none() {
            // this nums is already the last permutation
            return false;
        }

        let index = index.unwrap();

        let mut just_greater_than = index + 1;
        for i in just_greater_than..nums.len() {
            if nums[just_greater_than] > nums[i] && nums[i] > nums[index] {
                just_greater_than = i;
            }
        }
        nums.swap(index, just_greater_than);
        nums.get_mut((index + 1)..).unwrap().reverse();
        true
    }
}

fn main() {
    Solution::permute(vec![1, 2, 3]);

    let mut a = vec![2, 3, 1];
    Solution::next_permutation(&mut a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );

        assert_eq!(
            Solution::permute(vec![2, 1, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );

        assert_eq!(Solution::permute(vec![]), Vec::<Vec<i32>>::new());

        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);

        assert_eq!(Solution::permute(vec![1, 2]), vec![vec![1, 2], vec![2, 1]]);
    }
}
