mod decimal;
mod fraction;
mod macros;
mod ranges;
mod variant;

pub use decimal::*;
pub use fraction::*;
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
        pub digitSequenceLengthRange: Option<UnsignedRangeDto>,
        pub decimalSettings: Option<DecimalSettingsDto>,
        pub deltaTime: bool
    }
}
