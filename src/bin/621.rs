struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if tasks.len() <= 1 {
            return tasks.len() as i32;
        }
        let mut task_counts = vec![0; 26];
        let mut last_appear = vec![-1000000i32; 26];
        for i in 0..tasks.len() {
            task_counts[letter_to_index(tasks[i])] += 1;
        }

        let mut ans = 0;
        let mut rest_tasks = tasks.len();
        let mut cur_interval = 0;
        while rest_tasks > 0 {
            let sorted_counts = sort_counts(&task_counts);
            for i in (0..26).rev() {
                let (idx, cnt) = sorted_counts[i];
                if cnt == 0 {
                    break;
                }
                if cur_interval > n + last_appear[idx] {
                    task_counts[idx] -= 1;
                    last_appear[idx] = cur_interval;
                    rest_tasks -= 1;
                    break;
                }
            }
            ans += 1;
            cur_interval += 1;
        }
        ans
    }
}

fn letter_to_index(c: char) -> usize {
    ((c as u8) - ('A' as u8)) as usize
}

fn sort_counts(task_counts: &[i32]) -> Vec<(usize, i32)> {
    let mut task_counts = task_counts.iter().copied().enumerate().collect::<Vec<_>>();
    task_counts.sort_by_key(|a| a.1);
    task_counts
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_interval() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
        assert_eq!(Solution::least_interval(vec!['A', 'A', 'A'], 100), 203);
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'B', 'B', 'A', 'C'], 3),
            9
        );
        assert_eq!(Solution::least_interval(vec![], 3), 0);
        assert_eq!(Solution::least_interval(vec!['A'], 3), 1);
    }
}
