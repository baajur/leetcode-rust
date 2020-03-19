struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        use std::cmp::max;
        use std::collections::HashSet;
        let mut words_per_len = vec![vec![]; 16];
        let mut max_word_len = 1;
        let mut max_len = -1;

        for i in 0..words.len() {
            let c = words[i].chars().collect::<Vec<_>>();
            max_word_len = max(max_word_len, c.len());
            words_per_len[c.len() - 1].push(c);
        }

        let mut visited = HashSet::new();

        for i in 0..max_word_len {
            max_len = max(
                max_len,
                dfs(&words_per_len, vec![' '; i], &mut visited) - i as i32,
            );
        }

        max_len
    }
}

fn dfs(
    words: &Vec<Vec<Vec<char>>>,
    cur_word: Vec<char>,
    visited: &mut std::collections::HashSet<Vec<char>>,
) -> i32 {
    let mut res = cur_word.len() as i32;
    if words.len() as i32 == res {
        return words.len() as i32;
    }
    if visited.contains(&cur_word) {
        return 0;
    } else {
        visited.insert(cur_word.clone());
    }

    for next_word in words[cur_word.len()].iter() {
        if is_chain(&cur_word, next_word) {
            res = std::cmp::max(res, dfs(words, next_word.clone(), visited));
        }
    }
    res
}

fn is_chain(prev: &[char], next: &[char]) -> bool {
    let prev = prev.iter().filter(|&&c| c != ' ').collect::<String>();
    if &prev == "" {
        return true;
    }
    for i in 0..=(next.len() - 1) {
        let tmp = next[..i].iter().collect::<String>()
            + next[i + 1..].iter().collect::<String>().as_str();
        if tmp == prev {
            return true;
        }
    }
    false
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_chain() {
        assert!(is_chain(&['a', 'b'], &['a', 'b', 'c']));
        assert!(is_chain(&[' ', ' ', ' ', 'a'], &['a', 'b']));
        assert!(is_chain(&[], &['a']));
        assert!(is_chain(&['a'], &['b', 'a']));
        assert!(!is_chain(&['a', 'b'], &['a', 'b', 'c', 'd']));
    }

    #[test]
    fn test_longest_str_chain() {
        assert_eq!(
            Solution::longest_str_chain(vec!["a".to_string(), "ba".to_string(),]),
            2
        );
        assert_eq!(
            Solution::longest_str_chain(vec![
                "a".to_string(),
                "b".to_string(),
                "ba".to_string(),
                "bca".to_string(),
                "bda".to_string(),
                "bdca".to_string()
            ]),
            4
        );
    }
}
