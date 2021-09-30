use crate::f1field::ZqField;
use crate::scalar;
use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

pub fn exp(f: &ZqField, base: &BigInt, e: &BigInt) -> BigInt {
    if e.is_zero() {
        return BigInt::one();
    }

    let n = scalar::bits(e);
    if n.len() == 0 {
        return BigInt::one();
    }

    let mut res = base.clone();
    let mut i = (n.len() - 2) as isize;
    while i >= 0 {
        res = f.square(&res);
        if n[i as usize] == BigInt::one() {
            res = f.mul(&res, &base);
        }
        i -= 1;
    }
    return res;
}
