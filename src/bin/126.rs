struct Solution;

// TODO: NOT COMPLETED!!
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::{HashMap, HashSet, VecDeque};
        let mut change_map = HashMap::new();
        for word in word_list.iter().map(|w| w.chars().collect::<Vec<_>>()) {
            for i in 0..word.len() {
                let mut key = word.clone();
                key[i] = '*';
                let entry = change_map.entry(key).or_insert(Vec::new());
                entry.push(word.clone());
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((vec![begin_word], HashSet::new()));

        let mut ret = Vec::new();

        let mut shortest_len = None;
        while !queue.is_empty() {
            let (ladder, visited) = queue.pop_front().unwrap();
            if shortest_len.is_some() && shortest_len.unwrap() < ladder.len() {
                break;
            }

            if ladder.last().unwrap() == &end_word {
                shortest_len = Some(ladder.len());
                ret.push(ladder);
                continue;
            }
            let word = ladder.last().unwrap().chars().collect::<Vec<_>>();
            for i in 0..word.len() {
                let mut masked = word.clone();
                masked[i] = '*';
                if let Some(next_words) = change_map.get(&masked) {
                    for next_word in next_words.iter() {
                        if visited.contains(next_word) {
                            continue;
                        }
                        let mut clone_visited = visited.clone();
                        clone_visited.insert(next_word.clone());
                        let mut clone_ladder = ladder.clone();
                        clone_ladder.push(next_word.iter().collect::<String>());
                        queue.push_back((clone_ladder, clone_visited));
                    }
                }
            }
        }

        ret
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_ladders() {
        assert_eq!(
            Solution::find_ladders(
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
            vec![
                vec![
                    "hit".to_string(),
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "cog".to_string()
                ],
                vec![
                    "hit".to_string(),
                    "hot".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ],
            ]
        );

        assert_eq!(
            Solution::find_ladders(
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
            Vec::<Vec<String>>::new()
        );
    }
}
