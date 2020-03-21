struct Solution;

use std::cmp::min;
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut ans = 0;
        let mut y = y;
        while y > x {
            ans += 1;
            if y % 2 == 0 {
                y /= 2;
            } else {
                y += 1;
            }
        }
        ans + x - y
    }

    pub fn bfs_mle_broken_calc(x: i32, y: i32) -> i32 {
        if x == y {
            return 0;
        }
        if x > y {
            return x - y;
        }
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((x, 0));
        let target = y;
        let mut ans = std::i32::MAX;
        while !queue.is_empty() {
            let (x, depth) = queue.pop_front().unwrap();
            if visited.contains(&x) {
                continue;
            }
            visited.insert(x);
            if x == target {
                ans = min(ans, depth);
                break;
            }
            if x > y {
                ans = min(ans, x - y + depth);
                continue;
            }
            if x > 2 {
                queue.push_back((x - 1, depth + 1));
            }
            queue.push_back((x * 2, depth + 1));
        }
        ans
    }
}

fn main() {
    Solution::broken_calc(5, 8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broken_calc() {
        assert_eq!(Solution::broken_calc(2, 3), 2);
        assert_eq!(Solution::broken_calc(5, 8), 2);
        assert_eq!(Solution::broken_calc(3, 10), 3);
        assert_eq!(Solution::broken_calc(1024, 1), 1023);
        assert_eq!(Solution::broken_calc(1, 1000000000), 39);
    }
}
