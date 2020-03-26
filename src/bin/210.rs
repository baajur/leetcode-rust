struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // check if prerequisites are directed acyclic graph or not
        let mut ans = Vec::new();

        let mut in0 = Vec::new();
        let mut ins = vec![0; num_courses as usize];
        let mut links = Vec::with_capacity(num_courses as usize);
        for _ in 0..(num_courses as usize) {
            links.push(Vec::new());
        }
        for i in 0..prerequisites.len() {
            let to = prerequisites[i][0] as usize;
            let from = prerequisites[i][1] as usize;
            links[from].push(to);
            ins[to] += 1;
        }

        for i in 0..(num_courses as usize) {
            if ins[i] == 0 {
                in0.push(i);
            }
        }

        while !in0.is_empty() {
            let a = in0.pop().unwrap();
            ans.push(a as i32);
            for &l in &links[a] {
                ins[l] -= 1;
                if ins[l] == 0 {
                    in0.push(l);
                }
            }
        }

        if ans.len() as i32 == num_courses {
            ans
        } else {
            vec![]
        }
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_order() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 2, 1, 3]
        );
        assert_eq!(
            Solution::find_order(3, vec![vec![1, 0], vec![2, 1], vec![0, 2]]),
            vec![]
        );
        assert_eq!(Solution::find_order(0, vec![]), vec![]);
    }
}
