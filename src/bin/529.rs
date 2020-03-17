struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut min_since_day_start: Vec<i32> = time_points
            .into_iter()
            .map(|t| {
                let mut split = t.split(":");
                let h: i32 = split.next().unwrap().parse().unwrap();
                let m: i32 = split.next().unwrap().parse().unwrap();
                h * 60 + m
            })
            .collect();
        min_since_day_start.sort();
        let mut ret = std::i32::MAX;
        use std::cmp::min;
        for i in 0..min_since_day_start.len() {
            if i == min_since_day_start.len() - 1 {
                ret = min(
                    ret,
                    24 * 60 + min_since_day_start[0] - min_since_day_start[i],
                );
            } else {
                ret = min(ret, min_since_day_start[i + 1] - min_since_day_start[i]);
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
    fn test_find_min_difference() {
        assert_eq!(
            Solution::find_min_difference(vec![
                "23:59".to_string(),
                "0:01".to_string(),
                "09:24".to_string()
            ]),
            2
        );

        assert_eq!(
            Solution::find_min_difference(vec![
                "23:59".to_string(),
                "0:01".to_string(),
                "09:24".to_string(),
                "19:24".to_string(),
                "22:24".to_string(),
                "09:23".to_string(),
            ]),
            1
        );
    }
}
