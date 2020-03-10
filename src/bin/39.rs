struct Solution;

impl Solution {
    // Dynamic programming
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // dp[i] := combinations that sum to `i`
        // NOTE: In combinations, numbers must be sorted in ascending order for uniqueness
        let mut dp = vec![vec![]; target as usize + 1];

        // initialize
        for &c in candidates.iter() {
            if dp.len() <= c as usize {
                continue;
            }
            dp[c as usize].push(vec![c]);
        }

        for i in 0..=(target as usize) {
            for &c in candidates.iter() {
                match i.checked_sub(c as usize) {
                    Some(x) => {
                        for mut comb in dp[x].clone().into_iter() {
                            if comb.last().unwrap() <= &c {
                                comb.push(c);
                                dp[i].push(comb);
                            }
                        }
                    }
                    None => (),
                }
            }
        }

        dp[target as usize].clone()
    }
}

fn main() {
    Solution::combination_sum(vec![2, 3, 6, 7], 7);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![2, 2, 3]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }
}
