use crate::webify;
use std::ops::RangeInclusive;

webify! {
    pub struct SignedRange {
        pub min: i128,
        pub max: i128,
    }
}

impl Into<RangeInclusive<i128>> for &SignedRange {
    fn into(self) -> RangeInclusive<i128> {
        self.min..=self.max
    }
}

webify! {
    pub struct UnsignedRange {
        pub min: u128,
        pub max: u128
    }
}

impl Into<RangeInclusive<u128>> for &UnsignedRange {
    fn into(self) -> RangeInclusive<u128> {
        self.min..=self.max
    }
}
