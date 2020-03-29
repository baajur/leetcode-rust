struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        if rating.len() < 3 {
            return 0;
        }

        let mut ans = 0;
        for i in 0..(rating.len() - 2) {
            for j in (i + 1)..(rating.len() - 1) {
                for k in (j + 1)..rating.len() {
                    if (rating[i] < rating[j] && rating[j] < rating[k])
                        || (rating[i] > rating[j] && rating[j] > rating[k])
                    {
                        ans += 1;
                    }
                }
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
    fn test_num_teams() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
        assert_eq!(Solution::num_teams(vec![1]), 0);
    }
}
