use super::{SignedRange, UnsignedRange};
use crate::webify;

webify! {
    pub struct FractionSettings {
        pub denominator_range: UnsignedRange,
        pub numerator_range: SignedRange
    }
}
