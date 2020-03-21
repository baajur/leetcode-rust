struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut dists = Vec::with_capacity(points.len());
        for i in 0..points.len() {
            let d2 = points[i][0].pow(2u32) + points[i][1].pow(2u32);
            dists.push((d2, points[i][0], points[i][1]));
        }

        dists.sort_by_key(|a| a.0);
        dists
            .drain(..k as usize)
            .map(|(_, x, y)| vec![x, y])
            .collect()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_slosest() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
        assert_eq!(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }
}
