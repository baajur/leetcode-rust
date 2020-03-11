struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        for i in 0..2usize.pow(nums.len() as u32) {
            let mut tmp = Vec::new();

            for j in 0..nums.len() {
                if i & 1 << j != 0 {
                    tmp.push(nums[j]);
                }
            }
            ret.push(tmp);
        }
        ret
    }

    pub fn recurse_subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        ret.push(vec![]);
        recurse(&mut ret, vec![], nums);
        ret
    }
}

fn recurse(ret: &mut Vec<Vec<i32>>, mut subset: Vec<i32>, nums: Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    let &first = nums.first().unwrap();
    let rest: Vec<i32> = nums.get(1..).unwrap().iter().copied().collect();
    recurse(ret, subset.clone(), rest.clone());
    subset.push(first);
    ret.push(subset.clone());
    recurse(ret, subset, rest);
}

fn main() {
    dbg!(Solution::recurse_subsets(vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );

        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);

        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);

        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_recurse_subsets() {
        assert_eq!(
            Solution::recurse_subsets(vec![1, 2]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );

        assert_eq!(Solution::recurse_subsets(vec![1]), vec![vec![], vec![1]]);

        assert_eq!(Solution::recurse_subsets(vec![]), vec![vec![]]);

        assert_eq!(
            Solution::recurse_subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![2, 3],
                vec![1],
                vec![1, 3],
                vec![1, 2],
                vec![1, 2, 3]
            ]
        );
    }
}
