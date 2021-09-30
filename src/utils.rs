use num_bigint::{BigInt, Sign};
use sha3::{Digest, Keccak256};

pub fn str_keccak_bigint(seed: &str) -> BigInt {
    let mut hasher = Keccak256::new();
    hasher.update(seed.as_bytes());
    let result = hasher.finalize();
    return BigInt::from_bytes_be(Sign::Plus, &result[..]);
}

pub fn bigint_keccak_bigint(seed: &BigInt) -> BigInt {
    let (_, c_bytes) = seed.to_bytes_be();
    let mut c_bytes32: [u8; 32] = [0; 32];
    let diff = c_bytes32.len() - c_bytes.len();
    c_bytes32[diff..].copy_from_slice(&c_bytes[..]); // keccak need length in 32

    let mut hasher = Keccak256::new();
    hasher.update(&c_bytes32);
    let result = hasher.finalize();
    return BigInt::from_bytes_be(Sign::Plus, &result[..]);
}
