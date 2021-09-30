use mimc_hash::mimcsponge::hash;
use num_bigint::{BigInt, ToBigInt};

#[test]
fn test_hash() {
    let res1 = hash(
        &1.to_bigint().unwrap(),
        &2.to_bigint().unwrap(),
        &3.to_bigint().unwrap(),
    );
    assert_eq!(
        res1.0,
        "18444058245820418255538785847032978363886102372504864086197416499869253008979"
            .parse::<BigInt>()
            .unwrap()
    );
    assert_eq!(
        res1.1,
        "2646733164649743153031645792459389637917704265581895142760676293265176296759"
            .parse::<BigInt>()
            .unwrap()
    );
}
