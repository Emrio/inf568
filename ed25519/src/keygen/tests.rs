use super::*;

fn test_derive_public_key(private_key: &str, public_key: &str) {
    let private_key = hex::decode(private_key).unwrap().try_into().unwrap();

    let (computed_public_key, _, _) = derivate_public_key_scalar_prefix(private_key);
    let computed_public_key = hex::encode(computed_public_key);

    assert_eq!(computed_public_key, public_key);
}

#[test]
fn derive_public_key1() {
    test_derive_public_key(
        "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    );
}

#[test]
fn derive_public_key2() {
    test_derive_public_key(
        "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
        "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
    );
}

#[test]
fn derive_public_key3() {
    test_derive_public_key(
        "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
        "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
    );
}

#[test]
fn derive_public_key4() {
    test_derive_public_key(
        "f5e5767cf153319517630f226876b86c8160cc583bc013744c6bf255f5cc0ee5",
        "278117fc144c72340f67d0f2316e8386ceffbf2b2428c9c51fef7c597f1d426e",
    );
}

#[test]
fn derive_public_key5() {
    test_derive_public_key(
        "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42",
        "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
    );
}
