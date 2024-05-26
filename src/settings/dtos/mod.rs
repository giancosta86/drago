mod fractions;
mod ranges;
mod variant;

use crate::dto;

pub use fractions::*;
pub use ranges::*;
pub use variant::*;

dto! {
    pub struct SettingsDto {
        pub seed: u64,
        pub variant: Variant,
        pub integer_range: Option<SignedRangeDto>,
        pub fraction_settings: Option<FractionSettingsDto>,
    }
}
