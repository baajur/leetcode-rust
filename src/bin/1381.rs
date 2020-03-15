#[derive(Debug)]
struct CustomStack {
    max_size: usize,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            max_size: maxSize as usize,
            stack: Vec::with_capacity(maxSize as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() + 1 <= self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        match self.stack.pop() {
            None => -1,
            Some(x) => x,
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..std::cmp::min(self.stack.len(), k as usize) {
            self.stack[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut stack = CustomStack::new(4);
        assert_eq!(stack.stack.len(), 0);
        assert_eq!(stack.pop(), -1);
        stack.push(1);
        assert_eq!(stack.stack.len(), 1);
        stack.push(2);
        assert_eq!(stack.stack.len(), 2);
        stack.push(3);
        assert_eq!(stack.stack.len(), 3);
        stack.push(4);
        assert_eq!(stack.stack.len(), 4);
        stack.push(5);
        assert_eq!(stack.stack.len(), 4);
        assert_eq!(stack.pop(), 4);
        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.stack.len(), 2);
        stack.increment(1, 100);
        assert_eq!(stack.stack[0], 101);
        assert_eq!(stack.stack[1], 2);
        stack.increment(4, 100);
        assert_eq!(stack.stack[0], 201);
        assert_eq!(stack.stack[1], 102);
    }
}
