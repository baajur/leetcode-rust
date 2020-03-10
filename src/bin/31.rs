struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut last_greater_index = None;
        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] < nums[i + 1] {
                last_greater_index = Some(i);
                break;
            }
        }

        if last_greater_index.is_none() {
            // in case this sequence is the last permutaion
            nums.sort();
            return;
        }

        let last_greater_index = last_greater_index.unwrap();
        let mut next_lower_num_index = last_greater_index + 1;
        for i in last_greater_index..nums.len() {
            if nums[i] > nums[last_greater_index] && nums[i] < nums[next_lower_num_index] {
                next_lower_num_index = i;
            }
        }
        nums.swap(last_greater_index, next_lower_num_index);

        nums.get_mut((last_greater_index + 1)..).unwrap().sort();
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut vec = vec![1, 2, 3];
        Solution::next_permutation(&mut vec);
        assert_eq!(&vec, &vec![1, 3, 2]);

        let mut vec = vec![3, 2, 1];
        Solution::next_permutation(&mut vec);
        assert_eq!(&vec, &vec![1, 2, 3]);

        let mut vec = vec![1, 1, 5];
        Solution::next_permutation(&mut vec);
        assert_eq!(&vec, &vec![1, 5, 1]);

        let mut vec = vec![1, 3, 4, 2];
        Solution::next_permutation(&mut vec);
        assert_eq!(&vec, &vec![1, 4, 2, 3]);
    }
}
