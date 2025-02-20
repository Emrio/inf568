use std::str::FromStr;

use num_bigint::BigUint;

use crate::arithmetic::Point;

pub fn d() -> BigUint {
    BigUint::from_str(
        "37095705934669439343138083508754565189542113879843219016388785533085940283555",
    )
    .unwrap()
}

pub fn p() -> BigUint {
    BigUint::from(2u32).pow(255) - BigUint::from(19u32)
}

pub fn b() -> Point {
    let x = BigUint::from_str(
        "15112221349535400772501151409588531511454012693041857206046113283949847762202",
    )
    .unwrap();
    let y = BigUint::from_str(
        "46316835694926478169428394003475163141307993866256225615783033603165251855960",
    )
    .unwrap();
    Point::new(x, y)
}

pub fn l() -> BigUint {
    BigUint::from(2u32).pow(252)
        + BigUint::from_str("27742317777372353535851937790883648493").unwrap()
}
