mod currency;
mod decimal;
mod fraction;

use crate::{SettingsDto, SettingsError};
use chinese_format::CountBase;
use chinese_rand::gregorian::LinearTimeParams;
use std::ops::RangeInclusive;

pub use currency::*;
pub use decimal::*;
pub use fraction::*;

pub struct Settings {
    pub seed: u64,
    pub variant: chinese_format::Variant,
    pub integer_range: Option<RangeInclusive<i128>>,
    pub fraction_settings: Option<FractionSettings>,
    pub count_range: Option<RangeInclusive<CountBase>>,
    pub digit_sequence_length_range: Option<RangeInclusive<u8>>,
    pub decimal_settings: Option<DecimalSettings>,
    pub linear_time: Option<LinearTimeParams>,
    pub delta_time: bool,
}

impl TryFrom<SettingsDto> for Settings {
    type Error = SettingsError;

    fn try_from(dto: SettingsDto) -> Result<Self, Self::Error> {
        let integer_range = dto
            .integerRange
            .map(|dto| {
                let range: RangeInclusive<i128> = dto.try_into().map_err(SettingsError::Integer)?;
                Ok(range)
            })
            .transpose()?;

        let fraction_settings = dto
            .fractionSettings
            .map(|dto| {
                let settings: FractionSettings = dto.try_into().map_err(SettingsError::Fraction)?;
                Ok(settings)
            })
            .transpose()?;

        let count_range = dto
            .countRange
            .map(|dto| {
                let range: RangeInclusive<CountBase> =
                    dto.try_into().map_err(SettingsError::Count)?;
                Ok(range)
            })
            .transpose()?;

        let digit_sequence_length_range = dto
            .digitSequenceLengthRange
            .map(|dto| {
                let range: RangeInclusive<u8> =
                    dto.try_into().map_err(SettingsError::DigitSequenceLength)?;
                Ok(range)
            })
            .transpose()?;

        let decimal_settings = dto
            .decimalSettings
            .map(|dto| {
                let settings: DecimalSettings = dto.try_into().map_err(SettingsError::Decimal)?;
                Ok(settings)
            })
            .transpose()?;

        let linear_time_params: Option<LinearTimeParams> = dto.linearTime.map(|dto| dto.into());

        Ok(Settings {
            seed: dto.seed,
            variant: dto.variant.into(),
            integer_range,
            fraction_settings,
            count_range,
            digit_sequence_length_range,
            decimal_settings,
            linear_time: linear_time_params,
            delta_time: dto.deltaTime,
        })
    }
}
