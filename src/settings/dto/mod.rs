mod decimal;
mod fraction;
mod linear_time;
mod macros;
mod money;
mod ranges;
mod variant;

pub use decimal::*;
pub use fraction::*;
pub use linear_time::*;
pub use money::*;
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
        pub linearTime: Option<LinearTimeSettingsDto>,
        pub deltaTime: bool
    }
}
