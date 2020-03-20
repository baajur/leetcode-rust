struct Solution;

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        if t.is_empty() {
            return vec![];
        }
        if t.len() == 1 {
            return vec![0];
        }

        let mut w = vec![-1; 101];
        let mut ans = Vec::with_capacity(t.len());

        for i in (0..t.len()).rev() {
            if w[t[i] as usize] == -1 {
                ans.push(0);
            } else {
                ans.push(w[t[i] as usize] - i as i32);
            }

            for j in 0..t[i] {
                w[j as usize] = i as i32;
            }
        }

        ans.reverse();
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
    fn daily_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![100, 99, 98]),
            vec![0, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![50, 30, 70]),
            vec![2, 1, 0]
        );
    }
}
