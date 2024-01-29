pub trait CheckedCeilingDiv {
    fn checked_ceiling_div(self, divisor: i128) -> Option<i128>;
}

impl CheckedCeilingDiv for i128 {
    fn checked_ceiling_div(self, divisor: i128) -> Option<i128> {
        let result = self.checked_div(divisor)?;
        if self % divisor != 0 {
            result.checked_add(1)
        } else {
            Some(result)
        }
    }
}