struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.iter().any(|w| w == &end_word) {
            return 0;
        }

        use std::collections::{HashMap, HashSet, VecDeque};

        // pre-process
        let mut pp: HashMap<String, Vec<String>> = HashMap::new();
        for word in word_list.iter() {
            for i in 0..word.len() {
                let s = word.as_str();
                let key = s[0..i].to_string() + "*" + &s[(i + 1)..];
                let entry = pp.entry(key).or_insert(vec![]);
                entry.push(s.to_string());
            }
        }

        let mut q = VecDeque::new();
        let mut used = HashSet::new();
        q.push_back((begin_word, 1));

        while !q.is_empty() {
            let (cur_word, len) = q.pop_front().unwrap();
            if cur_word == end_word {
                return len;
            }
            for i in 0..cur_word.len() {
                let s = cur_word.as_str();
                let key = s[0..i].to_string() + "*" + &s[(i + 1)..];
                if let Some(next_words) = pp.get(&key) {
                    for next in next_words.iter() {
                        if !used.contains(next) {
                            q.push_back((next.to_string(), len + 1));
                            used.insert(next);
                        }
                    }
                }
            }
        }
        0
    }

    pub fn bfs_ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.iter().any(|w| w == &end_word) {
            return 0;
        }
        // BFS
        use std::collections::{HashSet, VecDeque};
        let word_list: HashSet<_> = word_list
            .into_iter()
            .map(|w| w.chars().collect::<Vec<_>>())
            .collect();
        let mut q = VecDeque::new();

        let begin_word: Vec<char> = begin_word.chars().collect();
        let end_word: Vec<char> = end_word.chars().collect();
        q.push_back((begin_word, word_list, 1));

        while !q.is_empty() {
            let (cur_word, word_list, len) = q.pop_front().unwrap();
            if dist(&cur_word, &end_word) == 0 {
                dbg!(&word_list);
                return len;
            }
            if word_list.is_empty() {
                continue;
            }

            for next_word in word_list.iter().filter(|w| dist(&w, &cur_word) == 1) {
                let mut next_word_list = word_list.clone();
                next_word_list.remove(next_word);
                q.push_back((next_word.clone(), next_word_list, len + 1));
            }
        }

        0
    }
}

fn dist(x: &[char], y: &[char]) -> i32 {
    assert!(x.len() == y.len());
    let mut ret = 0;
    for i in 0..x.len() {
        if x[i] != y[i] {
            ret += 1;
        }
    }
    ret
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladder_length() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );

        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                ]
            ),
            0
        );

        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );

        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "lxt".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            0
        );
    }

    #[test]
    fn test_bfs_ladder_length() {
        assert_eq!(
            Solution::bfs_ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );

        assert_eq!(
            Solution::bfs_ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                ]
            ),
            0
        );

        assert_eq!(
            Solution::bfs_ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );

        assert_eq!(
            Solution::bfs_ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "lxt".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            0
        );
    }
}
