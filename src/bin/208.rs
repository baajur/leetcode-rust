const ALPHABET_SIZE: usize = 26;
#[derive(Debug)]
struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        let children = {
            let mut v = Vec::with_capacity(ALPHABET_SIZE);
            for _ in 0..ALPHABET_SIZE {
                v.push(None);
            }
            v
        };
        Self {
            children,
            is_end_of_word: false,
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let chars = word.chars().collect::<Vec<_>>();
        let mut pointer = &mut self.root;
        for i in 0..chars.len() {
            let d = char_to_digit(chars[i]);
            if pointer.children[d].is_none() {
                pointer.children[d] = Some(Box::new(TrieNode::new()));
            }
            pointer = pointer.children[d].as_mut().unwrap();

            if i == chars.len() - 1 {
                pointer.is_end_of_word = true;
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let char_digits: Vec<_> = word.chars().map(|c| char_to_digit(c)).collect();
        let mut pointer = &self.root;
        for i in 0..char_digits.len() {
            let d = char_digits[i];
            if pointer.children[d].is_none() {
                return false;
            }
            pointer = pointer.children[d].as_ref().unwrap();
        }

        pointer.is_end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let char_digits: Vec<_> = prefix.chars().map(|c| char_to_digit(c)).collect();
        let mut pointer = &self.root;
        for i in 0..char_digits.len() {
            let d = char_digits[i];
            if pointer.children[d].is_none() {
                return false;
            }
            pointer = pointer.children[d].as_ref().unwrap();
        }
        true
    }
}

fn char_to_digit(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    dbg!(&trie);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
        assert!(!trie.search("orange".to_string()));
        assert!(!trie.starts_with("orange".to_string()));
    }
}
