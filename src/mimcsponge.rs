use crate::f1field::ZqField;
use crate::scalar;
use crate::utils::{bigint_keccak_bigint, str_keccak_bigint};
use lazy_static::lazy_static;
use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

pub const SEED: &str = "mimcsponge";
pub const NROUNDS: usize = 220;
lazy_static! {
    static ref F: ZqField = ZqField::new(&scalar::from_string(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617"
    ));
    static ref GLOBAL_CTS: Vec<BigInt> = get_constants(SEED, NROUNDS);
}

pub fn get_constants(seed: &str, n_rounds: usize) -> Vec<BigInt> {
    let mut cts: Vec<BigInt> = Vec::new();
    cts.push(F.e(&BigInt::zero()));

    let mut c = str_keccak_bigint(seed);
    for _ in 1..(n_rounds - 1) {
        c = bigint_keccak_bigint(&c);

        let n1 = &c % F.p();
        cts.push(F.e(&n1));
    }
    cts.push(F.e(&BigInt::zero()));
    cts
}

pub fn hash(_x_l_in: &BigInt, _x_r_in: &BigInt, _k: &BigInt) -> (BigInt, BigInt) {
    let mut x_l = F.e(_x_l_in);
    let mut x_r = F.e(_x_r_in);
    let k: BigInt = F.e(_k);
    for i in 0..NROUNDS {
        let c: &BigInt = &GLOBAL_CTS[i];
        let t = if i == 0 {
            F.add(&x_l, &k)
        } else {
            F.add(&F.add(&x_l, &k), c)
        };
        let x_r_tmp = F.e(&x_r);
        if i < (NROUNDS - 1) {
            x_r = x_l;
            x_l = F.add(&x_r_tmp, &F.pow(&t, &5.to_bigint().unwrap()));
        } else {
            x_r = F.add(&x_r_tmp, &F.pow(&t, &5.to_bigint().unwrap()));
        }
    }
    return (F.normalize(&x_l), F.normalize(&x_r));
}
