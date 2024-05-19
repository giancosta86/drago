use super::DigitSequenceLength;
use chinese_format::IntegerPart;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecimalParams {
    pub integer_range: RangeInclusive<IntegerPart>,
    pub fractional_length_range: RangeInclusive<DigitSequenceLength>,
}
