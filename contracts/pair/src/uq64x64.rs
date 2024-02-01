const Q64: u128 = 2u128.pow(64);


// encode a u64 as a UQ64x64
pub(crate) fn encode(y: u64) -> u128 {
    let y_into128: u128 = y.into();
    let z: u128 = y_into128 * Q64;
    z
}

// returns a UQ64x64 which represents the ratio of the x to y
pub(crate) fn fraction(x: u64, y: u64) -> u128 {
    if y == 0 {
        panic!("DIV_BY_ZERO")
    }
    uqdiv(encode(x),y)
}


// divide a UQ64x64 by a u64, returning a UQ64x64
pub(crate) fn uqdiv(x: u128, y: u64) -> u128 {
    if y == 0 {
        panic!("DIV_BY_ZERO")
    }

    let y_into128: u128 = y.into();
    let z: u128 = x / y_into128;
    z
}


