struct Solution;

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        let mut cur_elem = arr[0];
        let mut cnt = 0;
        let mut ans = 0;
        for i in 0..arr.len() {
            if arr[i] == cur_elem {
                cnt += 1;
            } else if arr[i] == cur_elem + 1 {
                ans += cnt;
                cnt = 1;
                cur_elem = arr[i];
            } else {
                cnt = 1;
                cur_elem = arr[i];
            }
        }
        ans
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_elements() {
        assert_eq!(Solution::count_elements(vec![1, 1, 2, 2]), 2);
        assert_eq!(Solution::count_elements(vec![1, 1, 2, 2, 2, 3]), 5);
        assert_eq!(Solution::count_elements(vec![1, 2, 3]), 2);
        assert_eq!(Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
    }
}
