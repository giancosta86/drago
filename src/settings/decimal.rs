use chinese_format::IntegerPart;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecimalSettings {
    pub integer_range: RangeInclusive<IntegerPart>,
    pub fractional_length_range: RangeInclusive<u8>,
}
