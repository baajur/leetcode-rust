struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut iter = data.into_iter();
        while let Some(b) = iter.next() {
            let bytes = {
                let mut cnt = 0;
                for i in (0..8).rev() {
                    if 1 << i & b != 0 {
                        cnt += 1;
                    } else {
                        break;
                    }
                }
                cnt
            };
            if bytes == 0 {
                // 1-byte character
                continue;
            }
            if bytes == 1 || bytes >= 5 {
                return false;
            }

            // multi-byte character
            for _ in 0..(bytes - 1) {
                match iter.next() {
                    None => return false,
                    Some(b) => {
                        if (b & 1 << 7 != 0) && (b & 1 << 6 == 0) {
                            // does this byte starts with `10`?
                            continue;
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_utf8() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
        assert!(Solution::valid_utf8(vec![127]));
        assert!(!Solution::valid_utf8(vec![145]));
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
        assert!(!Solution::valid_utf8(vec![250, 145, 145, 145, 145]));
    }
}
