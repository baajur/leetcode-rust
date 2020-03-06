struct Solution;

impl Solution {
    pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
        let mut ret = Vec::new();
        let mut dict = dict;
        dict.sort_by_key(|d| d.len());
        'outer: for word in sentence.split(" ") {
            for root in dict.iter() {
                if word.starts_with(root) {
                    ret.push(root.clone());
                    continue 'outer;
                }
            }
            ret.push(word.to_string());
        }
        ret.join(" ")
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_words() {
        let dict = vec![
            "cate".to_string(),
            "cat".to_string(),
            "bat".to_string(),
            "rat".to_string(),
        ];
        let sentence = "category the cattle was rattled by the battery".to_string();
        assert_eq!(
            Solution::replace_words(dict, sentence),
            "cat the cat was rat by the bat".to_string()
        );
    }
}
