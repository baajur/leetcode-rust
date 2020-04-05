struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut ans = Vec::new();

        while let Some((h, ch)) = heap.pop() {
            let l = ans.len();
            if l >= 2 && ans[l - 1] == ch && ans[l - 2] == ch {
                match heap.pop() {
                    Some((hh, ch2)) => {
                        ans.push(ch2);
                        if hh > 1 {
                            heap.push((hh - 1, ch2));
                        }
                        heap.push((h, ch));
                        continue;
                    }
                    None => return ans.into_iter().collect::<String>(),
                }
            }

            ans.push(ch);
            if h > 1 {
                heap.push((h - 1, ch));
            }
        }

        ans.into_iter().collect::<String>()
    }

    pub fn tle_longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        if a + b + c == 1 {
            if a == 1 {
                return "a".to_string();
            }
            if b == 1 {
                return "b".to_string();
            }
            if c == 1 {
                return "c".to_string();
            }
        }
        let mut max_ret = vec![];
        if a >= 2 {
            helper('a', 'a', &mut max_ret, a - 2, b, c);
        }
        if b >= 2 {
            helper('b', 'b', &mut max_ret, a, b - 2, c);
        }
        if c >= 2 {
            helper('c', 'c', &mut max_ret, a, b, c - 2);
        }

        if a >= 1 {
            if b >= 1 {
                helper('a', 'b', &mut max_ret, a - 1, b - 1, c);
            }
            if c >= 1 {
                helper('a', 'c', &mut max_ret, a - 1, b, c - 1);
            }
        }
        if b >= 1 {
            if a >= 1 {
                helper('b', 'a', &mut max_ret, a - 1, b - 1, c);
            }
            if c >= 1 {
                helper('b', 'c', &mut max_ret, a, b - 1, c - 1);
            }
        }
        if c >= 1 {
            if a >= 1 {
                helper('c', 'a', &mut max_ret, a - 1, b, c - 1);
            }
            if b >= 1 {
                helper('c', 'b', &mut max_ret, a, b - 1, c - 1);
            }
        }

        max_ret.into_iter().collect::<String>()
    }
}

fn helper(one: char, two: char, ret: &mut Vec<char>, a: i32, b: i32, c: i32) {
    let tmp = dfs(vec![one, two], one, two, a, b, c);
    if ret.len() < tmp.len() {
        *ret = tmp;
    }
}

fn dfs(
    cur_s: Vec<char>,
    prev2: char,
    prev1: char,
    rest_a: i32,
    rest_b: i32,
    rest_c: i32,
) -> Vec<char> {
    let mut max_ret = cur_s.clone();
    if prev2 == prev1 {
        if prev1 == 'a' && rest_b == 0 && rest_c == 0 {
            return cur_s;
        }
        if prev1 == 'b' && rest_a == 0 && rest_c == 0 {
            return cur_s;
        }
        if prev1 == 'c' && rest_a == 0 && rest_b == 0 {
            return cur_s;
        }

        if prev1 == 'a' {
            if rest_b > 0 {
                let mut cl = cur_s.clone();
                cl.push('b');
                let tmp = dfs(cl, prev1, 'b', rest_a, rest_b - 1, rest_c);
                if max_ret.len() < tmp.len() {
                    max_ret = tmp;
                }
            }
            if rest_c > 0 {
                let mut cl = cur_s.clone();
                cl.push('c');
                let tmp = dfs(cl, prev1, 'c', rest_a, rest_b, rest_c - 1);
                if max_ret.len() < tmp.len() {
                    max_ret = tmp;
                }
            }
        }

        if prev1 == 'b' {
            if rest_a > 0 {
                let mut cl = cur_s.clone();
                cl.push('a');
                let tmp = dfs(cl, prev1, 'a', rest_a - 1, rest_b, rest_c);
                if max_ret.len() < tmp.len() {
                    max_ret = tmp;
                }
            }
            if rest_c > 0 {
                let mut cl = cur_s.clone();
                cl.push('c');
                let tmp = dfs(cl, prev1, 'c', rest_a, rest_b, rest_c - 1);
                if max_ret.len() < tmp.len() {
                    max_ret = tmp;
                }
            }
        }

        if prev1 == 'c' {
            if rest_b > 0 {
                let mut cl = cur_s.clone();
                cl.push('b');
                let tmp = dfs(cl, prev1, 'b', rest_a, rest_b - 1, rest_c);
                if max_ret.len() < tmp.len() {
                    max_ret = tmp;
                }
            }
            if rest_a > 0 {
                let mut cl = cur_s.clone();
                cl.push('a');
                let tmp = dfs(cl, prev1, 'a', rest_a - 1, rest_b, rest_c);
                if max_ret.len() < tmp.len() {
                    max_ret = tmp;
                }
            }
        }

        return max_ret;
    }

    if rest_a > 0 {
        let mut cl = cur_s.clone();
        cl.push('a');
        let tmp = dfs(cl, prev1, 'a', rest_a - 1, rest_b, rest_c);
        if max_ret.len() < tmp.len() {
            max_ret = tmp;
        }
    }
    if rest_b > 0 {
        let mut cl = cur_s.clone();
        cl.push('b');
        let tmp = dfs(cl, prev1, 'b', rest_a, rest_b - 1, rest_c);
        if max_ret.len() < tmp.len() {
            max_ret = tmp;
        }
    }
    if rest_c > 0 {
        let mut cl = cur_s.clone();
        cl.push('c');
        let tmp = dfs(cl, prev1, 'c', rest_a, rest_b, rest_c - 1);
        if max_ret.len() < tmp.len() {
            max_ret = tmp;
        }
    }
    max_ret
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_diverse_string() {
        assert_eq!(Solution::longest_diverse_string(1, 1, 7), "ccbccacc");
        assert_eq!(Solution::longest_diverse_string(2, 2, 1), "bacba");
        assert_eq!(Solution::longest_diverse_string(7, 1, 0), "aabaa");
        assert_eq!(Solution::longest_diverse_string(1, 1, 0), "ba");
        assert_eq!(Solution::longest_diverse_string(1, 0, 0), "a");
        assert_eq!(Solution::longest_diverse_string(1, 1, 1), "cba");
    }

    #[test]
    fn test_tle_longest_diverse_string() {
        assert_eq!(Solution::tle_longest_diverse_string(1, 1, 7), "ccbccacc");
        assert_eq!(Solution::tle_longest_diverse_string(2, 2, 1), "aabbc");
        assert_eq!(Solution::tle_longest_diverse_string(7, 1, 0), "aabaa");
        assert_eq!(Solution::tle_longest_diverse_string(1, 1, 0), "ab");
        assert_eq!(Solution::tle_longest_diverse_string(1, 0, 0), "a");
        assert_eq!(Solution::tle_longest_diverse_string(1, 1, 1), "abc");
    }
}
