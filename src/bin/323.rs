struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut visited = HashSet::new();

        let mut links = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            let n1 = edges[i][0] as usize;
            let n2 = edges[i][1] as usize;
            links[n1].push(n2);
            links[n2].push(n1);
        }

        let mut ret = 0;
        for i in 0..n as usize {
            if visited.contains(&i) {
                continue;
            }
            dfs(&mut visited, &links, i);
            ret += 1;
        }
        ret
    }
}

fn dfs(visited: &mut std::collections::HashSet<usize>, links: &Vec<Vec<usize>>, start: usize) {
    if visited.contains(&start) {
        return;
    }

    visited.insert(start);
    for &to in links[start].iter() {
        dfs(visited, links, to);
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_components() {
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]),
            2
        );

        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]),
            1
        );

        assert_eq!(Solution::count_components(0, vec![]), 0);
    }
}
