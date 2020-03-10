struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let sorted = strs
            .iter()
            .map(|s| {
                let mut x = s.chars().collect::<Vec<_>>();
                x.sort();
                x.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>();

        for i in 0..sorted.len() {
            let entry = map.entry(&sorted[i]).or_insert(vec![]);
            entry.push(i);
        }

        let mut res = Vec::new();
        for (_, value) in map.into_iter() {
            let mut tmp = Vec::new();
            for v in value {
                tmp.push(strs[v].clone());
            }
            res.push(tmp);
        }

        res
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![vec!["ate", "eat", "tea"], vec!["nat", "tan"], vec!["bat"]]
        );
    }
}
