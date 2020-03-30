struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // brute force
        let mut gas2 = Vec::with_capacity(gas.len() * 2);
        let mut cost2 = Vec::with_capacity(cost.len() * 2);
        for i in 0..gas.len() {
            gas2.push(gas[i]);
            cost2.push(cost[i]);
        }
        for i in 0..gas.len() {
            gas2.push(gas[i]);
            cost2.push(cost[i]);
        }

        'outer: for s in 0..gas.len() {
            let mut cur_gas = 0;
            for i in s..(s + gas.len()) {
                cur_gas += gas2[i];
                if cur_gas - cost2[i] < 0 {
                    continue 'outer;
                }
                cur_gas -= cost2[i]
            }
            return s as i32;
        }
        -1
    }

    pub fn can_complete_circuit_onepass(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut curr_tank = 0;
        let mut total_tank = 0;
        for i in 0..gas.len() {
            if curr_tank < 0 {
                start = i;
                curr_tank = 0;
            }
            curr_tank += gas[i];
            total_tank += gas[i];
            curr_tank -= cost[i];
            total_tank -= cost[i];
        }
        if total_tank >= 0 {
            start as i32
        } else {
            -1
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
    fn test_can_complete_circuit() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn test_can_complete_circuit_onepass() {
        assert_eq!(
            Solution::can_complete_circuit_onepass(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit_onepass(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
