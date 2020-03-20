struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
        let mut index = 0;
        calc(&s, &mut index)
    }
}

fn calc(s: &[char], index: &mut usize) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let mut val = read_val(s, index);
    while *index < s.len() {
        match s[*index] {
            '*' => {
                *index += 1;
                val *= read_val(s, index);
            }
            '/' => {
                *index += 1;
                val /= read_val(s, index);
            }
            '+' => {
                *index += 1;
                val += read_and_calc_high_precedence(s, index);
            }
            '-' => {
                *index += 1;
                val -= read_and_calc_high_precedence(s, index);
            }
            _ => unreachable!(),
        }
    }
    val
}

fn read_and_calc_high_precedence(s: &[char], index: &mut usize) -> i32 {
    // calculate * and /
    let mut val = read_val(s, index);
    while *index < s.len() {
        match s[*index] {
            '*' => {
                *index += 1;
                val *= read_val(s, index);
            }
            '/' => {
                *index += 1;
                val /= read_val(s, index);
            }
            _ => break,
        }
    }
    val
}

fn read_val(s: &[char], index: &mut usize) -> i32 {
    let mut val = (s[*index] as u8 - '0' as u8) as i32;
    *index += 1;
    while *index < s.len() {
        if s[*index].is_ascii_digit() {
            val = val * 10 + (s[*index] as u8 - '0' as u8) as i32;
            *index += 1;
        } else {
            break;
        }
    }
    val
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_val() {
        let s: Vec<char> = "3+2*2".chars().collect();
        let mut index = 0;
        assert_eq!(read_val(&s, &mut index), 3);
        assert_eq!(index, 1);

        let s: Vec<char> = "33/2".chars().collect();
        let mut index = 0;
        assert_eq!(read_val(&s, &mut index), 33);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
        assert_eq!(Solution::calculate("3/2".to_string()), 1);
        assert_eq!(Solution::calculate("3+5 / 2".to_string()), 5);
        assert_eq!(Solution::calculate("1-0+5*2/3".to_string()), 4);
        assert_eq!(Solution::calculate(" 3+4*5-2/         2".to_string()), 22);
    }
}
