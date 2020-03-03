struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // check if this relation of courses is Directed Acyclic Graph or not
        #[derive(Debug, Clone)]
        struct Node {
            num_inflows: i32,
            outflows: Vec<usize>,
        }
        impl Node {
            fn init() -> Self {
                Self {
                    num_inflows: 0,
                    outflows: Vec::new(),
                }
            }
        }

        let mut nodes = vec![Node::init().clone(); num_courses as usize];
        for p in prerequisites {
            let from = p[0] as usize;
            let to = p[1] as usize;
            nodes[to].num_inflows += 1;
            nodes[from].outflows.push(to);
        }

        let mut no_inflow_nodes: Vec<_> = nodes
            .iter()
            .enumerate()
            .filter_map(
                |(i, node)| {
                    if node.num_inflows == 0 {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
            .collect();

        let mut sort_result = Vec::new();

        while !no_inflow_nodes.is_empty() {
            let popped = no_inflow_nodes.pop().unwrap();
            sort_result.push(popped);
            for to_node in nodes[popped].outflows.clone().into_iter() {
                nodes[to_node].num_inflows -= 1;
                if nodes[to_node].num_inflows == 0 {
                    no_inflow_nodes.push(to_node);
                }
            }
        }

        sort_result.len() == num_courses as usize
    }
}

fn main() {
    let result = Solution::can_finish(2, vec![vec![0, 1]]);
    let result2 = Solution::can_finish(2, vec![vec![0, 1], vec![1, 0]]);
    dbg!(result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
