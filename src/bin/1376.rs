struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut subs = vec![vec![]; n as usize];
        for i in 0..(n as usize) {
            if manager[i] != -1 {
                subs[manager[i] as usize].push(i as i32);
            }
        }
        dfs(head_id, &subs, &inform_time, 0)
    }
}

fn dfs(emp_id: i32, subs: &Vec<Vec<i32>>, inform_time: &[i32], cur_time: i32) -> i32 {
    if subs[emp_id as usize].is_empty() {
        return cur_time;
    }
    let mut max_time = -1;
    let cur_time = cur_time + inform_time[emp_id as usize];
    for &s in subs[emp_id as usize].iter() {
        max_time = std::cmp::max(max_time, dfs(s, subs, inform_time, cur_time));
    }
    max_time
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_minutes() {
        let tests = [
            (1, 0, vec![-1], vec![0], 0),
            (6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0], 1),
            (
                7,
                6,
                vec![1, 2, 3, 4, 5, 6, -1],
                vec![0, 6, 5, 4, 3, 2, 1],
                21,
            ),
            (
                8,
                0,
                vec![-1, 5, 0, 6, 7, 0, 0, 0],
                vec![89, 0, 0, 0, 0, 523, 241, 519],
                612,
            ),
        ];
        for test in tests.iter() {
            let n = test.0;
            let head_id = test.1;
            let manager = test.2.clone();
            let inform_time = test.3.clone();
            let expected = test.4;
            assert_eq!(
                Solution::num_of_minutes(n, head_id, manager, inform_time),
                expected
            );
        }
    }
}
