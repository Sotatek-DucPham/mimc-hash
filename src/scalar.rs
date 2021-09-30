use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

pub fn from_string(s: &str) -> BigInt {
    s.parse::<BigInt>().unwrap()
}

pub fn bits(n: &BigInt) -> Vec<BigInt> {
    let mut res: Vec<BigInt> = Vec::new();
    let mut e = n.clone();
    while &e > &BigInt::zero() {
        if &e & BigInt::one() == BigInt::one() {
            res.push(BigInt::one());
        } else {
            res.push(BigInt::zero());
        }
        e /= 2;
    }
    res
}
