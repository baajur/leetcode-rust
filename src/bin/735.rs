struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        'outer: for a in asteroids {
            if stack.is_empty() {
                stack.push(a);
            } else {
                while !stack.is_empty() && a < 0 && 0 < stack[stack.len() - 1] {
                    if a.abs() > stack[stack.len() - 1] {
                        stack.pop();
                    } else if a.abs() == stack[stack.len() - 1] {
                        stack.pop();
                        continue 'outer;
                    } else {
                        continue 'outer;
                    }
                }
                stack.push(a);
            }
        }
        stack
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asteroid_collision() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
    }
}
