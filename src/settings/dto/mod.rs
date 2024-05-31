mod fractions;
mod macros;
mod ranges;
mod variant;

pub use fractions::*;
pub use ranges::*;
pub use variant::*;

use crate::dto;

dto! {
    pub struct SettingsDto {
        pub seed: u64,
        pub variant: Variant,
        pub integerRange: Option<SignedRangeDto>,
        pub fractionSettings: Option<FractionSettingsDto>,
        pub countRange: Option<UnsignedRangeDto>,
        pub digitSequenceLengthRange: Option<UnsignedRangeDto>
    }
}
