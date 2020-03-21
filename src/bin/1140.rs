struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        let mut acc_sum = Vec::new();
        for i in (0..piles.len()).rev() {
            if acc_sum.is_empty() {
                acc_sum.push(piles[i]);
            } else {
                let last = acc_sum.last().unwrap();
                acc_sum.push(piles[i] + last);
            }
        }
        acc_sum.reverse();
        dp(1, 0, &piles, &mut memo, &acc_sum)
    }
}

fn dp(
    cur_m: usize,
    index: usize,
    piles: &[i32],
    memo: &mut HashMap<(usize, usize), i32>,
    acc_sum: &[i32],
) -> i32 {
    if let Some(&v) = memo.get(&(cur_m, index)) {
        return v;
    }

    if index >= piles.len() {
        memo.insert((cur_m, index), 0);
        return 0;
    }

    let rest = acc_sum[index];
    if piles.len() - index <= 2 * cur_m {
        // take all rest stones
        memo.insert((cur_m, index), rest);
        return rest;
    }

    use std::cmp::max;
    let mut max_val = -1;
    for x in 1..=(2 * cur_m) {
        max_val = max(
            max_val,
            rest - dp(max(cur_m, x), index + x, piles, memo, acc_sum),
        );
    }
    memo.insert((cur_m, index), max_val);
    max_val
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_game_ii() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
        assert_eq!(Solution::stone_game_ii(vec![2, 7]), 9);
        assert_eq!(Solution::stone_game_ii(vec![1]), 1);
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 3]), 9);
        assert_eq!(
            Solution::stone_game_ii(vec![
                3111, 4303, 2722, 2183, 6351, 5227, 8964, 7167, 9286, 6626, 2347, 1465, 5201, 7240,
                5463, 8523, 8163, 9391, 8616, 5063, 7837, 7050, 1246, 9579, 7744, 6932, 7704, 9841,
                6163, 4829, 7324, 6006, 4689, 8781, 621
            ]),
            112766
        );
    }
}
