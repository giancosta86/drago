mod fractions;
mod ranges;
mod variant;

use crate::webify;

pub use fractions::*;
pub use ranges::*;
pub use variant::*;

webify! {
    pub struct LogogramGeneratorSettings {
        pub seed: u64,
        pub variant: Variant,
        pub integer_range: Option<SignedRange>,
        pub fraction_settings: Option<FractionSettings>
    }
}
