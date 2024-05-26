use crate::dto;
use std::ops::RangeInclusive;

dto! {
    pub struct SignedRangeDto {
        pub min: i128,
        pub max: i128,
    }
}

impl Into<RangeInclusive<i128>> for &SignedRangeDto {
    fn into(self) -> RangeInclusive<i128> {
        self.min..=self.max
    }
}

dto! {
    pub struct UnsignedRangeDto {
        pub min: u128,
        pub max: u128
    }
}

impl Into<RangeInclusive<u128>> for &UnsignedRangeDto {
    fn into(self) -> RangeInclusive<u128> {
        self.min..=self.max
    }
}
