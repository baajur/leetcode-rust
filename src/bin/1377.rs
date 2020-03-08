struct Solution;

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut subs = vec![vec![]; (n + 1) as usize];
        for i in 0..(n as usize - 1) {
            let from = edges[i][0];
            let to = edges[i][1];
            subs[from as usize].push(to);
            subs[to as usize].push(from);
        }
        use std::collections::{HashSet, VecDeque};
        let mut q = VecDeque::new();
        q.push_back((1, 1.0, 0));
        let mut visited = HashSet::new();
        visited.insert(1);

        while !q.is_empty() {
            let (cur, prev_prob, depth) = q.pop_front().unwrap();
            if depth > t {
                break;
            }
            let num_sub = subs[cur].iter().filter(|s| !visited.contains(s)).count();
            if cur as i32 == target {
                if depth == t {
                    return prev_prob;
                } else if num_sub == 0 {
                    return prev_prob;
                } else {
                    return 0.0;
                }
            }
            for &s in subs[cur].iter() {
                if visited.contains(&s) {
                    continue;
                }
                q.push_back((s as usize, prev_prob * 1.0 / num_sub as f64, depth + 1));
                visited.insert(s);
            }
        }

        0.0
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frog_position() {
        assert_eq!(
            Solution::frog_position(
                7,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 7],
                    vec![2, 4],
                    vec![2, 6],
                    vec![3, 5]
                ],
                2,
                4
            ),
            0.16666666666666666
        );

        assert_eq!(
            Solution::frog_position(3, vec![vec![2, 1], vec![3, 2],], 1, 2),
            1.0
        );
    }
}
