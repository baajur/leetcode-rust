struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let calc_square_sum = |x: u32| {
            x.to_string()
                .chars()
                .map(|a| (a as u32 - '0' as u32).pow(2u32))
                .sum::<u32>()
        };
        let mut cur_number = calc_square_sum(n as u32);
        let mut already_appear = HashSet::new();

        while cur_number != 1 && !already_appear.contains(&cur_number) {
            already_appear.insert(cur_number);
            cur_number = calc_square_sum(cur_number);
        }

        cur_number == 1
    }
}

fn main() {
    Solution::is_happy(19);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(Solution::is_happy(1));
        assert!(!Solution::is_happy(2));
        assert!(!Solution::is_happy(3));
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(20));
    }
}
