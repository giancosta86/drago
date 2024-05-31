use std::ops::RangeInclusive;

//TODO: are all these public fields necessary?
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FractionSettings {
    pub denominator_range: RangeInclusive<u128>,
    pub numerator_range: RangeInclusive<i128>,
}
