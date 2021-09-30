use lazy_static::lazy_static;
use mimc_hash::f1field::ZqField;
use mimc_hash::scalar;
use num_bigint::{BigInt, ToBigInt};

lazy_static! {
    static ref F: ZqField = ZqField::new(&scalar::from_string(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617"
    ));
}

#[test]
fn test_f1field() {
    let zero = F.e(&0.to_bigint().unwrap());
    assert_eq!(zero, 0.to_bigint().unwrap());

    let mod_res = "112262945085563849078138178410332731715548306547361811415364261237931404627739"
        .parse::<BigInt>()
        .unwrap()
        % F.p();
    assert_eq!(
        mod_res,
        "2821730726367472966906149684046356272806484545281639696873240305052362149654"
            .parse::<BigInt>()
            .unwrap()
    );
}
