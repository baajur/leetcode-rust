struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // we are not allowed to divide :(
        if nums.len() <= 1 {
            return vec![];
        }
        let mut acc_prd = Vec::with_capacity(nums.len() + 1);
        let mut acc_prd_rev = Vec::with_capacity(nums.len() + 1);
        acc_prd.push(1);
        acc_prd_rev.push(1);

        for i in 0..nums.len() {
            acc_prd.push(acc_prd[i] * nums[i]);
            acc_prd_rev.push(acc_prd_rev[i] * nums[nums.len() - 1 - i]);
        }

        (0..nums.len())
            .map(|i| acc_prd[i] * acc_prd_rev[nums.len() - 1 - i])
            .collect()
    }

    pub fn constant_space_product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // we are not allowed to divide :(
        // space complexity: O(1)
        if nums.len() <= 1 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            let last = *ret.last().unwrap_or(&1);
            ret.push(last * nums[i]);
        }

        let mut right = 1;
        for i in (0..nums.len()).rev() {
            if i == 0 {
                ret[0] = right;
            } else {
                ret[i] = ret[i - 1] * right;
                right *= nums[i];
            }
        }
        ret
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(Solution::product_except_self(vec![1, 2]), vec![2, 1]);
        assert!(Solution::product_except_self(vec![]).is_empty());
        assert!(Solution::product_except_self(vec![1]).is_empty());
    }

    #[test]
    fn constant_space_product_except_self() {
        assert_eq!(
            Solution::constant_space_product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::constant_space_product_except_self(vec![1, 2]),
            vec![2, 1]
        );
        assert!(Solution::constant_space_product_except_self(vec![]).is_empty());
        assert!(Solution::constant_space_product_except_self(vec![1]).is_empty());
    }
}
