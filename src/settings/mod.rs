mod dto_macros;
mod error;
mod fractions;
mod ranges;
mod variant;

use std::ops::RangeInclusive;

use crate::dto;

use chinese_format::Fraction;
pub use fractions::*;
pub use ranges::*;
pub use variant::*;

use self::error::SettingsError;

pub struct Settings {
    pub seed: u64,
    pub variant: chinese_format::Variant,
    pub integer_range: Option<RangeInclusive<i128>>,
    pub fraction_settings: Option<FractionSettings>,
}

dto! {
    pub struct SettingsDto {
        pub seed: u64,
        pub variant: Variant,
        pub integer_range: Option<SignedRangeDto>,
        pub fraction_settings: Option<FractionSettingsDto>,
    }
}

impl TryFrom<SettingsDto> for Settings {
    type Error = SettingsError;

    fn try_from(source: SettingsDto) -> Result<Self, Self::Error> {
        let integer_range = source
            .integer_range
            .map(|dto| {
                let range: RangeInclusive<i128> =
                    dto.try_into().map_err(|message| SettingsError {
                        message,
                        source: error::ErrorSource::Integer,
                    })?;
                Ok(range)
            })
            .transpose()?;

        let fraction_settings = source
            .fraction_settings
            .map(|dto| {
                let settings: FractionSettings = (&dto).try_into()?;
                Ok(settings)
            })
            .transpose()?;

        Ok(Settings {
            seed: source.seed,
            variant: source.variant.into(),
            integer_range,
            fraction_settings,
        })
    }
}
