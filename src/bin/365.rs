struct Solution;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if z < 0 || x + y < z {
            return false;
        }

        use std::cmp::{max, min};
        use std::collections::{HashSet, VecDeque};
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((0, 0));
        while !queue.is_empty() {
            let (a, b) = queue.pop_front().unwrap();
            if visited.contains(&(a, b)) {
                continue;
            }

            if a + b == z {
                return true;
            }
            visited.insert((a, b));

            queue.push_back((x, b)); // fill jug 1
            queue.push_back((a, y)); // fill jug 2
            queue.push_back((0, b)); // dump jug 1
            queue.push_back((a, 0)); // dump jug 2
            queue.push_back((max(0, a - (y - b)), min(y, b + a))); // pour from jug 1 to jug 2
            queue.push_back((min(x, a + b), max(0, b - (x - a)))); // pour from jug 2 to jug 1
        }

        false
    }

    pub fn can_measure_water2(x: i32, y: i32, z: i32) -> bool {
        use std::collections::HashSet;
        let mut visited = HashSet::new();
        dfs(0, 0, x, y, z, &mut visited)
    }
}

fn dfs(
    a: i32,
    b: i32,
    x: i32,
    y: i32,
    z: i32,
    visited: &mut std::collections::HashSet<(i32, i32)>,
) -> bool {
    if visited.contains(&(a, b)) {
        return false;
    }
    if a + b == z {
        dbg!(a, b);
        return true;
    }
    visited.insert((a, b));

    if dfs(x, b, x, y, z, visited) {
        dbg!(a, b);
        return true;
    }

    if dfs(a, y, x, y, z, visited) {
        dbg!(a, b);
        return true;
    }

    if dfs(0, b, x, y, z, visited) {
        dbg!(a, b);
        return true;
    }

    if dfs(a, 0, x, y, z, visited) {
        dbg!(a, b);
        return true;
    }

    use std::cmp::{max, min};
    if dfs(max(0, a - (y - b)), min(y, b + a), x, y, z, visited) {
        dbg!(a, b);
        return true;
    }

    if dfs(min(x, a + b), max(0, b - (x - a)), x, y, z, visited) {
        dbg!(a, b);
        return true;
    }

    false
}

fn main() {
    Solution::can_measure_water(3, 5, 4);
    Solution::can_measure_water2(3, 5, 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_measure_water() {
        assert!(Solution::can_measure_water(3, 5, 4));
        assert!(!Solution::can_measure_water(2, 6, 5));
    }

    #[test]
    fn test_can_measure_water2() {
        assert!(Solution::can_measure_water2(3, 5, 4));
        assert!(!Solution::can_measure_water2(2, 6, 5));
    }
}
