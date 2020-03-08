struct Solution;

impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut h = BinaryHeap::new();
        let mut ans = 0;
        for i in 0..light.len() {
            h.push(light[i]);
            if *h.peek().unwrap() == i as i32 + 1 {
                ans += 1;
            }
        }
        ans
    }

    pub fn segtree_num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut st = SegmentTree::new(0, light.len(), |a, b| a + b);

        let mut ans = 0;

        for k in 0..light.len() {
            st.update(light[k] as usize - 1, 1);
            if st.query(0, k + 1) == k + 1 {
                ans += 1;
            }
        }
        ans
    }
}

#[derive(Debug)]
pub struct SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    values: Vec<T>,
    n_leaves: usize,
    identity_elem: T,
    func: F,
}
impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(init_value: T, size: usize, func: F) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            values: vec![init_value; 2 * n - 1],
            n_leaves: n,
            identity_elem: init_value,
            func,
        }
    }
    pub fn update(&mut self, leaf_number: usize, replace_value: T) {
        let mut node_number = leaf_number + self.n_leaves - 1;
        self.values[node_number] = replace_value;
        while node_number > 0 {
            node_number = (node_number - 1) / 2;
            self.values[node_number] = (self.func)(
                self.values[node_number * 2 + 1],
                self.values[node_number * 2 + 2],
            );
        }
    }
    pub fn query(&self, begin: usize, end: usize) -> T {
        self.internal_query(begin, end, 0, 0, self.n_leaves)
    }
    fn internal_query(
        &self,
        begin: usize,
        end: usize,
        node_number: usize,
        left: usize,
        right: usize,
    ) -> T {
        if right <= begin || end <= left {
            self.identity_elem
        } else if begin <= left && right <= end {
            self.values[node_number]
        } else {
            let c1 = self.internal_query(begin, end, 2 * node_number + 1, left, (left + right) / 2);
            let c2 =
                self.internal_query(begin, end, 2 * node_number + 2, (left + right) / 2, right);
            (self.func)(c1, c2)
        }
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_times_all_blue() {
        let tests = [
            (vec![2, 1, 3, 5, 4], 3),
            (vec![3, 2, 4, 1, 5], 2),
            (vec![4, 1, 2, 3], 1),
            (vec![2, 1, 4, 3, 6, 5], 3),
            (vec![1, 2, 3, 4, 5, 6], 6),
        ];

        for test in tests.iter() {
            let input = test.0.clone();
            let expected = test.1;
            assert_eq!(Solution::segtree_num_times_all_blue(input), expected);
        }

        for test in tests.iter() {
            let input = test.0.clone();
            let expected = test.1;
            assert_eq!(Solution::num_times_all_blue(input), expected);
        }
    }
}
