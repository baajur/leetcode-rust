use std::collections::BTreeSet;

struct ExamRoom {
    seats: usize,
    state: BTreeSet<usize>,
}

impl ExamRoom {
    fn new(N: i32) -> Self {
        Self {
            seats: N as usize,
            state: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.state.is_empty() {
            self.state.insert(0);
            0
        } else {
            let mut ret = 0;
            let mut dist = 0;
            let mut it = self.state.iter().peekable();
            // see left empty seats from the first student
            if let Some(&&first) = it.peek() {
                dist = first + 1;
            }
            while let Some(&idx) = it.next() {
                match it.peek() {
                    Some(&next) => {
                        let tmp = next - idx;
                        let tmp = if tmp % 2 == 0 { tmp } else { tmp - 1 };
                        if tmp > dist {
                            dist = next - idx;
                            ret = idx + (next - idx) / 2;
                        }
                    }
                    None => {
                        if (self.seats - idx) > dist {
                            dist = self.seats - idx;
                            ret = self.seats - 1;
                        }
                    }
                }
            }
            self.state.insert(ret);
            ret as i32
        }
    }

    fn leave(&mut self, p: i32) {
        self.state.remove(&(p as usize));
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exam_room() {
        let mut er = ExamRoom::new(10);
        assert_eq!(er.seat(), 0);
        assert_eq!(er.seat(), 9);
        assert_eq!(er.seat(), 4);
        assert_eq!(er.seat(), 2);
        er.leave(4);
        assert_eq!(er.seat(), 5);
        assert_eq!(er.seat(), 7);

        let mut er = ExamRoom::new(10);
        assert_eq!(er.seat(), 0);
        assert_eq!(er.seat(), 9);
        assert_eq!(er.seat(), 4);
        er.leave(0);
        er.leave(4);
        assert_eq!(er.seat(), 0);
        assert_eq!(er.seat(), 4);
        assert_eq!(er.seat(), 2);
        assert_eq!(er.seat(), 6);
        assert_eq!(er.seat(), 1);
        assert_eq!(er.seat(), 3);
        assert_eq!(er.seat(), 5);
        assert_eq!(er.seat(), 7);
        assert_eq!(er.seat(), 8);
        er.leave(0);
        er.leave(4);
        assert_eq!(er.seat(), 0);
        assert_eq!(er.seat(), 4);

        let mut er = ExamRoom::new(4);
        assert_eq!(er.seat(), 0);
        assert_eq!(er.seat(), 3);
        assert_eq!(er.seat(), 1);
        assert_eq!(er.seat(), 2);
        er.leave(3);
        er.leave(0);
        er.leave(2);
        assert_eq!(er.seat(), 3);
    }

    #[test]
    fn test_exam_room2() {
        let input1 = [
            "ExamRoom", "seat", "seat", "seat", "leave", "leave", "seat", "seat", "seat", "seat",
            "seat", "seat", "seat", "seat", "seat", "leave", "leave", "seat", "seat", "leave",
            "seat", "leave", "seat", "leave", "seat", "leave", "seat", "leave", "leave", "seat",
            "seat", "leave", "leave", "seat", "seat", "leave",
        ];
        let input2 = [
            [10],
            [-1],
            [-1],
            [-1],
            [0],
            [4],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [-1],
            [0],
            [4],
            [-1],
            [-1],
            [7],
            [-1],
            [3],
            [-1],
            [3],
            [-1],
            [9],
            [-1],
            [0],
            [8],
            [-1],
            [-1],
            [0],
            [8],
            [-1],
            [-1],
            [2],
        ];
        let expected = [
            -1, 0, 9, 4, -1, -1, 0, 4, 2, 6, 1, 3, 5, 7, 8, -1, -1, 0, 4, -1, 7, -1, 3, -1, 3, -1,
            9, -1, -1, 0, 8, -1, -1, 0, 8, -1,
        ];

        let mut er = ExamRoom::new(input2[0][0]);
        for i in 1..input1.len() {
            if input1[i] == "seat" {
                assert_eq!(er.seat(), expected[i]);
            } else {
                er.leave(input2[i][0]);
            }
        }
    }
}
