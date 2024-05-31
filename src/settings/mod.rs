mod decimal;
#[allow(non_snake_case)]
mod dto;
mod error;
mod fraction;
mod ranges;
mod variant;

use chinese_format::CountBase;
use std::ops::RangeInclusive;

pub use decimal::*;
pub use dto::*;
pub use error::*;
pub use fraction::*;

pub struct Settings {
    pub seed: u64,
    pub variant: chinese_format::Variant,
    pub integer_range: Option<RangeInclusive<i128>>,
    pub fraction_settings: Option<FractionSettings>,
    pub count_range: Option<RangeInclusive<CountBase>>,
    pub digit_sequence_length_range: Option<RangeInclusive<u8>>,
    pub decimal_settings: Option<DecimalSettings>,
}

impl TryFrom<SettingsDto> for Settings {
    type Error = SettingsError;

    fn try_from(source: SettingsDto) -> Result<Self, Self::Error> {
        let integer_range = source
            .integerRange
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
            .fractionSettings
            .map(|dto| {
                let settings: FractionSettings = (&dto).try_into()?;
                Ok(settings)
            })
            .transpose()?;

        let count_range = source
            .countRange
            .map(|dto| {
                let range: RangeInclusive<CountBase> =
                    dto.try_into().map_err(|message| SettingsError {
                        message,
                        source: error::ErrorSource::Count,
                    })?;
                Ok(range)
            })
            .transpose()?;

        let digit_sequence_length_range = source
            .digitSequenceLengthRange
            .map(|dto| {
                let range: RangeInclusive<u8> =
                    dto.try_into().map_err(|message| SettingsError {
                        message,
                        source: error::ErrorSource::DigitSequence,
                    })?;
                Ok(range)
            })
            .transpose()?;

        let decimal_settings = source
            .decimalSettings
            .map(|dto| {
                let settings: DecimalSettings = (&dto).try_into()?;
                Ok(settings)
            })
            .transpose()?;

        Ok(Settings {
            seed: source.seed,
            variant: source.variant.into(),
            integer_range,
            fraction_settings,
            count_range,
            digit_sequence_length_range,
            decimal_settings,
        })
    }
}
