struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = Vec::new();
        let mut a: Vec<u32> = a.chars().map(|c| c as u32 - '0' as u32).collect();
        let mut b: Vec<u32> = b.chars().map(|c| c as u32 - '0' as u32).collect();
        a.reverse();
        b.reverse();
        let mut carry = 0;
        for i in 0..std::cmp::max(a.len(), b.len()) {
            let &v1 = a.get(i).unwrap_or(&0);
            let &v2 = b.get(i).unwrap_or(&0);
            let sum = v1 + v2 + carry;
            res.push(std::char::from_digit(sum % 2, 10).unwrap());
            carry = sum / 2;
        }
        if carry > 0 {
            res.push(std::char::from_digit(carry, 10).unwrap());
        }
        res.reverse();
        res.into_iter().collect()
    }

    pub fn naive_add_binary(a: String, b: String) -> String {
        let a = u64::from_str_radix(a.as_str(), 2).unwrap();
        let b = u64::from_str_radix(b.as_str(), 2).unwrap();
        format!("{:b}", a + b)
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        );
        assert_eq!(Solution::add_binary("0".to_string(), "0".to_string()), "0");
        assert_eq!(
            Solution::add_binary(
                "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
                "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()),
                "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000"
        );
    }
}
