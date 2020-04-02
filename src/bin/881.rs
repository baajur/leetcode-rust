struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut p = people;
        p.sort_unstable();

        let mut ans = 0;
        let mut left = 0i32;
        let mut right = (p.len() - 1) as i32;
        while left <= right {
            if p[left as usize] + p[right as usize] <= limit {
                ans += 1;
                left += 1;
                right -= 1;
            } else {
                ans += 1;
                right -= 1;
            }
        }
        ans
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rescue_boats() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}
