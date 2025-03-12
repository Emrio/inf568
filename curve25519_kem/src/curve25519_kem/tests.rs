use crate::{decaps, encaps, keygen};

#[test]
fn test_alice_bob() {
    let (pk, sk) = keygen();

    let (c, k1) = encaps(pk);

    let k2 = decaps(c, sk);

    assert_eq!(k1, k2);
}
