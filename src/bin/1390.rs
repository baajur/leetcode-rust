struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let lower_primes = 400i32.lower_primes();
        for i in 0..nums.len() {
            let mut s = 0;
            let mut cnt = 1;
            if nums[i] == 1 {
                continue;
            }
            let f = factorize(nums[i], &lower_primes);
            if f.len() >= 3 {
                continue;
            }
            if f.len() == 1 {
                for (k, v) in f {
                    if v != 3 {
                        break;
                    }
                    ans += (k * k + 1) * (k + 1);
                }
            } else {
                // f.len == 2
                let mut stmp = Vec::new();
                let mut ok = true;
                for (k, v) in f {
                    if v != 1 {
                        ok = false;
                        break;
                    }
                    stmp.push(k);
                }
                if ok {
                    ans += 1 + stmp[0] + stmp[1] + stmp[0] * stmp[1];
                }
            }
        }

        ans
    }
}

fn factorize(v: i32, primes: &[i32]) -> std::collections::HashMap<i32, usize> {
    let mut ret = std::collections::HashMap::new();
    let mut tmp = v;
    for &prime in primes {
        if v < prime * prime {
            break;
        }
        while tmp % prime == 0 {
            tmp = tmp / prime;
            *ret.entry(prime).or_insert(0) += 1;
        }
    }
    if tmp > 1 {
        *ret.entry(tmp).or_insert(0) += 1;
    }
    ret
}

fn main() {
    dbg!(Solution::sum_four_divisors(vec![21, 4, 7]));
    dbg!(Solution::sum_four_divisors(vec![21, 4, 7, 1]));
    dbg!(Solution::sum_four_divisors(vec![21, 4, 7, 1, 27]));
    dbg!(Solution::sum_four_divisors(vec![21, 4, 7, 1, 9]));
}

pub trait Int:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::hash::Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
    fn next(self) -> Self;
    fn prev(self) -> Self;
    fn sqrt_floor(self) -> Self;
}
macro_rules ! impl_int_for_numerics { ( $ ( $ t : ty ) * ) => ( $ ( impl Int for $ t { fn zero ( ) -> Self { 0 } fn one ( ) -> Self { 1 } fn next ( self ) -> Self { self + Self :: one ( ) } fn prev ( self ) -> Self { self - Self :: one ( ) } fn sqrt_floor ( self ) -> Self { if self < Self :: zero ( ) { return Self :: zero ( ) ; } let two = Self :: one ( ) . next ( ) ; let mut ok = Self :: zero ( ) ; let mut ng = self . next ( ) ; while ng - ok > 1 { let mid = ( ng + ok ) / two ; if mid * mid <= self { ok = mid ; } else { ng = mid ; } } ok } } ) * ) }
impl_int_for_numerics ! ( u8 i8 u16 i16 u32 i32 u64 i64 usize isize ) ;
pub trait Prime<T: Int> {
    fn lower_primes(&self) -> Vec<T>;
    fn factorize(&self) -> std::collections::HashMap<T, usize>;
}
impl<T> Prime<T> for T
where
    T: Int,
{
    #[doc = " エラトステネスの篩を用いてself以下の素数を求める"]
    #[doc = " 計算量: O(n log log n)"]
    fn lower_primes(&self) -> Vec<T> {
        let &this = self;
        let mut v = Vec::new();
        if this <= T::one() {
            return v;
        }
        let mut deque = std::collections::VecDeque::new();
        let mut t = T::one().next();
        while t <= this {
            deque.push_back(t);
            t = t.next();
        }
        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p * p <= this {
            deque = deque
                .iter()
                .filter(|&&x| x % p != T::zero())
                .map(|x| *x)
                .collect();
            p = match deque.pop_front() {
                Some(x) => x,
                None => return v,
            };
            v.push(p);
        }
        for n in deque {
            v.push(n);
        }
        v
    }
    #[doc = " エラトステネスの篩を用いてselfを素因数分解する"]
    fn factorize(&self) -> std::collections::HashMap<T, usize> {
        let mut ret = std::collections::HashMap::new();
        let primes = self.sqrt_floor().lower_primes();
        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == T::zero() {
                tmp = tmp / prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > T::one() {
            *ret.entry(tmp).or_insert(0) += 1;
        }
        ret
    }
}
