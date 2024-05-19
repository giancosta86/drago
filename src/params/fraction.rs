use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FractionParams {
    pub denominator_range: RangeInclusive<u128>,
    pub numerator_range: RangeInclusive<i128>,
}
