use super::{
    error::{ErrorSource, SettingsError},
    FractionSettingsDto,
};
use std::ops::RangeInclusive;

//TODO: are all these public fields necessary?
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FractionSettings {
    pub denominator_range: RangeInclusive<u128>,
    pub numerator_range: RangeInclusive<i128>,
}

impl TryFrom<&FractionSettingsDto> for FractionSettings {
    type Error = SettingsError;

    fn try_from(dto: &FractionSettingsDto) -> Result<Self, Self::Error> {
        let denominator_range: RangeInclusive<u128> =
            dto.denominatorRange
                .try_into()
                .map_err(|message| SettingsError {
                    message,
                    source: ErrorSource::FractionDenominator,
                })?;

        if *denominator_range.start() == 0 {
            return Err(SettingsError {
                message: "The denominator is zero".to_string(),
                source: ErrorSource::FractionDenominator,
            }
            .into());
        }

        let numerator_range: RangeInclusive<i128> =
            dto.numeratorRange
                .try_into()
                .map_err(|message| SettingsError {
                    message,
                    source: ErrorSource::FractionNumerator,
                })?;

        Ok(FractionSettings {
            denominator_range,
            numerator_range,
        })
    }
}
