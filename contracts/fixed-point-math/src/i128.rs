use crate::fixed_point::FixedPoint;

impl FixedPoint for i128 {
    fn fixed_mul_floor(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_floor(self, y, denominator)
    }

    fn fixed_mul_ceil(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_ceil(self, y, denominator)
    }

    fn fixed_div_floor(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_floor(self, denominator, y)
    }

    fn fixed_div_ceil(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_ceil(self, denominator, y)
    }
}

// @dev TODO: Can optimize once `div_floor` and `div_ceil` are stable
//            https://github.com/rust-lang/rust/issues/88581

/// Performs floor(x * y / z)
fn mul_div_floor(x: i128, y: i128, z: i128) -> Option<i128> {
    let r = x.checked_mul(y)?;
    if r < 0 || (r > 0 && z < 0) {
        // ceiling is taken by default for a negative result
        let remainder = r.checked_rem_euclid(z)?;
        (r / z).checked_sub(if remainder > 0 { 1 } else { 0 })
    } else {
        // floor taken by default for a positive or zero result
        r.checked_div(z)
    }
}

/// Performs ceil(x * y / z)
fn mul_div_ceil(x: i128, y: i128, z: i128) -> Option<i128> {
    let r = x.checked_mul(y)?;
    if r <= 0 || (r > 0 && z < 0) {
        // ceiling is taken by default for a negative or zero result
        r.checked_div(z)
    } else {
        // floor taken by default for a positive result
        let remainder = r.checked_rem_euclid(z)?;
        (r / z).checked_add(if remainder > 0 { 1 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor_rounds_down() {
        let x: i128 = 1_5391283;
        let y: i128 = 314_1592653;
        let denominator: i128 = 1_0000001;

        let result = x.fixed_mul_floor(y, denominator).unwrap();

        assert_eq!(result, 483_5313675)
    }

    #[test]
    fn test_fixed_mul_floor_negative_rounds_down() {
        let x: i128 = -1_5391283;
        let y: i128 = 314_1592653;
        let denominator: i128 = 1_0000001;

        let result = x.fixed_mul_floor(y, denominator).unwrap();

        assert_eq!(result, -483_5313676)
    }

    #[test]
    fn test_fixed_mul_floor_large_number() {
        let x: i128 = 170_141_183_460_469_231_731;
        let y: i128 = 1_000_000_000_000_000_000;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_mul_floor(y, denominator).unwrap();

        assert_eq!(result, 170_141_183_460_469_231_731)
    }

    #[test]
    fn test_fixed_mul_floor_phantom_overflow() {
        let x: i128 = 170_141_183_460_469_231_731;
        let y: i128 = 1_000_000_000_000_000_001;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_mul_floor(y, denominator);

        assert_eq!(None, result);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil_rounds_up() {
        let x: i128 = 1_5391283;
        let y: i128 = 314_1592653;
        let denominator: i128 = 1_0000001;

        let result = x.fixed_mul_ceil(y, denominator).unwrap();

        assert_eq!(result, 483_5313676)
    }

    #[test]
    fn test_fixed_mul_ceil_negative_rounds_up() {
        let x: i128 = -1_5391283;
        let y: i128 = 314_1592653;
        let denominator: i128 = 1_0000001;

        let result = x.fixed_mul_ceil(y, denominator).unwrap();

        assert_eq!(result, -483_5313675)
    }

    #[test]
    fn test_fixed_mul_ceil_large_number() {
        let x: i128 = 170_141_183_460_469_231_731;
        let y: i128 = 1_000_000_000_000_000_000;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_mul_ceil(y, denominator).unwrap();

        assert_eq!(result, 170_141_183_460_469_231_731)
    }

    #[test]
    fn test_fixed_mul_ceil_phantom_overflow() {
        let x: i128 = 170_141_183_460_469_231_731;
        let y: i128 = 1_000_000_000_000_000_001;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_mul_ceil(y, denominator);

        assert_eq!(None, result);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor_rounds_down() {
        let x: i128 = 314_1592653;
        let y: i128 = 1_5391280;
        let denominator: i128 = 1_0000000;

        let result = x.fixed_div_floor(y, denominator).unwrap();

        assert_eq!(result, 204_1150997)
    }

    #[test]
    fn test_fixed_div_floor_negative_rounds_down() {
        let x: i128 = 314_1592653;
        let y: i128 = -1_5391280;
        let denominator: i128 = 1_0000000;

        let result = x.fixed_div_floor(y, denominator).unwrap();

        assert_eq!(result, -204_1150998)
    }

    #[test]
    fn test_fixed_div_floor_large_number() {
        let x: i128 = 170_141_183_460_469_231_731;
        let y: i128 = 1_000_000_000_000_000_000;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_div_floor(y, denominator).unwrap();

        assert_eq!(result, 170_141_183_460_469_231_731)
    }

    #[test]
    fn test_fixed_div_floor_phantom_overflow() {
        let x: i128 = 170_141_183_460_469_231_732;
        let y: i128 = 1_000_000_000_000_000_000;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_div_floor(y, denominator);

        assert_eq!(None, result);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil_rounds_down() {
        let x: i128 = 314_1592653;
        let y: i128 = 1_5391280;
        let denominator: i128 = 1_0000000;

        let result = x.fixed_div_ceil(y, denominator).unwrap();

        assert_eq!(result, 204_1150998)
    }

    #[test]
    fn test_fixed_div_ceil_negative_rounds_down() {
        let x: i128 = 314_1592653;
        let y: i128 = -1_5391280;
        let denominator: i128 = 1_0000000;

        let result = x.fixed_div_ceil(y, denominator).unwrap();

        assert_eq!(result, -204_1150997)
    }

    #[test]
    fn test_fixed_div_ceil_large_number() {
        let x: i128 = 170_141_183_460_469_231_731;
        let y: i128 = 1_000_000_000_000_000_000;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_div_ceil(y, denominator).unwrap();

        assert_eq!(result, 170_141_183_460_469_231_731)
    }

    #[test]
    fn test_fixed_div_ceil_phantom_overflow() {
        let x: i128 = 170_141_183_460_469_231_732;
        let y: i128 = 1_000_000_000_000_000_000;
        let denominator: i128 = 1_000_000_000_000_000_000;

        let result = x.fixed_div_ceil(y, denominator);

        assert_eq!(None, result);
    }
}
