use crate::futils;
use num_bigint::BigInt;
use num_traits::Zero;

pub struct ZqField {
    p: BigInt,
}

impl ZqField {
    pub fn new(p: &BigInt) -> ZqField {
        ZqField { p: p.clone() }
    }

    pub fn e(&self, a: &BigInt) -> BigInt {
        if a < &BigInt::zero() {
            let mut nres: BigInt = a * -1;
            if nres >= self.p {
                nres = nres % &self.p
            }
            return &self.p - nres;
        } else {
            return if a >= &self.p { a % &self.p } else { a * 1 };
        }
    }

    pub fn add(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let res = a + b;
        return if res >= self.p { res - &self.p } else { res };
    }

    pub fn mul(&self, a: &BigInt, b: &BigInt) -> BigInt {
        return (a * b) % &self.p;
    }

    pub fn square(&self, a: &BigInt) -> BigInt {
        return (a * a) % &self.p;
    }

    pub fn pow(&self, b: &BigInt, e: &BigInt) -> BigInt {
        return futils::exp(self, b, e);
    }

    pub fn normalize(&self, a: &BigInt) -> BigInt {
        if a < &BigInt::zero() {
            let mut na = a * -1;
            if na >= self.p {
                na = na % &self.p;
            }
            return &self.p - na;
        } else {
            return if a >= &self.p { a % &self.p } else { a.clone() };
        }
    }

    pub fn p(&self) -> &BigInt {
        &self.p
    }
}
