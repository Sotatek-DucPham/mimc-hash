use mimc_hash::mimcsponge::get_constants;
use mimc_hash::utils::{bigint_keccak_bigint, str_keccak_bigint};
use num_bigint::BigInt;

#[test]
fn test_keccak() {
    let seed = "mimcsponge";
    let mut c = str_keccak_bigint(seed);
    assert_eq!(
        c,
        "7382897923368232911732724532248874244391730858766999739770637613918792750368"
            .parse::<BigInt>()
            .unwrap()
    );

    c = bigint_keccak_bigint(&c);
    assert_eq!(
        c,
        "7120861356467848435263064379192047478074060781135320967663101236819528304084"
            .parse::<BigInt>()
            .unwrap()
    );

    c = bigint_keccak_bigint(&c);
    assert_eq!(
        c,
        "70689433897239714865317093925917138723043752151259405256620021708555425923532"
            .parse::<BigInt>()
            .unwrap()
    );

    // specific
    let result = bigint_keccak_bigint(
        &"148317947440800089795933930720822493695520852448386394775371401743494965187"
            .parse::<BigInt>()
            .unwrap(),
    );
    assert_eq!(
        result,
        "106554022159114821241876402108722893984444724481658840098429576635681621035524"
            .parse::<BigInt>()
            .unwrap()
    );
}

#[test]
fn test_cts() {
    let seed = "mimcsponge";
    let nrounds = 220;
    let cts = get_constants(seed, nrounds);

    assert_eq!(
        cts[1],
        "7120861356467848435263064379192047478074060781135320967663101236819528304084"
            .parse::<BigInt>()
            .unwrap()
    );
    assert_eq!(
        cts[10],
        "4798196928559910300796064665904583125427459076060519468052008159779219347957"
            .parse::<BigInt>()
            .unwrap()
    );
    assert_eq!(
        cts[50],
        "11501810868606870391127866188394535330696206817602260610801897042898616817272"
            .parse::<BigInt>()
            .unwrap()
    );
    assert_eq!(
        cts[100],
        "2821730726367472966906149684046356272806484545281639696873240305052362149654"
            .parse::<BigInt>()
            .unwrap()
    );
    assert_eq!(
        cts[200],
        "12216779172735125538689875667307129262237123728082657485828359100719208190116"
            .parse::<BigInt>()
            .unwrap()
    );
    assert_eq!(cts[219], "0".parse::<BigInt>().unwrap());
}
