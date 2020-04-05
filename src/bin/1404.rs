struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        use std::collections::VecDeque;
        let mut c: VecDeque<char> = s.chars().collect::<Vec<_>>().into();
        let mut cnt = 0;
        loop {
            if c.len() == 1 && c[0] == '1' {
                break;
            }

            cnt += 1;
            let last = c[c.len() - 1];
            if last == '0' {
                c.pop_back();
                continue;
            } else {
                let mut carry = 1;
                let l = c.len();
                c[l - 1] = '0';
                for i in (0..(l - 1)).rev() {
                    if carry == 0 {
                        break;
                    }
                    if c[i] == '1' {
                        c[i] = '0';
                        continue;
                    } else {
                        c[i] = '1';
                        carry = 0;
                        continue;
                    }
                }
                if carry == 1 {
                    c.push_front('1');
                }
            }
        }

        cnt
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_steps() {
        assert_eq!(Solution::num_steps("1101".to_string()), 6);
        assert_eq!(Solution::num_steps("10".to_string()), 1);
        assert_eq!(Solution::num_steps("1".to_string()), 0);
        assert_eq!(Solution::num_steps("1".repeat(500)), 501);
        assert_eq!(
            Solution::num_steps("100000000000000000000000000000".to_string()),
            29
        );
    }
}
