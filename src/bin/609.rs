struct Solution;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map = HashMap::new(); // key is content of a file, value is vector of file paths

        for p in paths {
            let mut iter = p.split_whitespace();
            let dir = iter.next().unwrap().to_string();
            while let Some(file) = iter.next() {
                let mut inner_iter = file.split("(");
                let filename = inner_iter.next().unwrap();
                let content = inner_iter.next().unwrap();
                map.entry(content.to_string())
                    .or_insert_with(Vec::new)
                    .push(format!("{}/{}", &dir, filename));
            }
        }
        map.into_iter()
            .filter_map(|(_, v)| if v.len() >= 2 { Some(v) } else { None })
            .collect()
    }
}

fn main() {
    dbg!(Solution::find_duplicate(vec![
        "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
        "root/c 3.txt(abcd)".to_string(),
        "root/c/d 4.txt(efgh)".to_string(),
        "root 4.txt(efgh)".to_string(),
    ]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        let mut result = Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
            "root/c 3.txt(abcd)".to_string(),
            "root/c/d 4.txt(efgh)".to_string(),
            "root 4.txt(efgh)".to_string(),
        ]);
        result.iter_mut().for_each(|r| r.sort());
        result.sort();
        assert_eq!(
            result,
            vec![
                vec!["root/4.txt", "root/a/2.txt", "root/c/d/4.txt"],
                vec!["root/a/1.txt", "root/c/3.txt"],
            ]
        );

        assert!(Solution::find_duplicate(vec![]).is_empty());

        // no duplicate
        assert!(Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efsfgh)".to_string(),
            "root/c 3.txt(abdfcd)".to_string(),
            "root/c/d 4.txt(efggdfh)".to_string(),
        ])
        .is_empty());
    }
}
