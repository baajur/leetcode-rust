struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut ret = Vec::new();
        let mut cur_val = None;
        let mut from_val = None;
        for i in 0..nums.len() {
            let replace = match cur_val {
                None => (Some(nums[i]), Some(nums[i])),
                Some(prev_val) => {
                    if nums[i] == prev_val + 1 {
                        (from_val, Some(nums[i]))
                    } else {
                        let from = from_val.unwrap();
                        if from == prev_val {
                            ret.push(format!("{}", prev_val));
                        } else {
                            ret.push(format!("{}->{}", from_val.unwrap(), prev_val));
                        }
                        (Some(nums[i]), Some(nums[i]))
                    }
                }
            };
            cur_val = replace.1;
            from_val = replace.0;
        }
        let from = from_val.unwrap();
        let to = cur_val.unwrap();
        if from == to {
            ret.push(format!("{}", to));
        } else {
            ret.push(format!("{}->{}", from, to));
        }

        ret
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        let test_cases = [
            (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
            (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
            (
                vec![0, 2, 3, 4, 6, 8, 9, 100, 102, 103, 104, 105],
                vec!["0", "2->4", "6", "8->9", "100", "102->105"],
            ),
            (vec![], vec![]),
        ];

        for test in test_cases.iter() {
            assert_eq!(Solution::summary_ranges(test.0.clone()), test.1);
        }
    }
}
