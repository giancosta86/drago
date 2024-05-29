use std::ops::RangeInclusive;

use super::{
    error::{ErrorSource, SettingsError},
    SignedRangeDto, UnsignedRangeDto,
};
use crate::dto;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FractionSettings {
    pub denominator_range: RangeInclusive<u128>,
    pub numerator_range: RangeInclusive<i128>,
}

dto! {
    pub struct FractionSettingsDto {
        pub denominator_range: UnsignedRangeDto,
        pub numerator_range: SignedRangeDto
    }
}

impl TryFrom<&FractionSettingsDto> for FractionSettings {
    type Error = SettingsError;

    fn try_from(source: &FractionSettingsDto) -> Result<Self, Self::Error> {
        let denominator_range: RangeInclusive<u128> =
            source
                .denominator_range
                .try_into()
                .map_err(|message| SettingsError {
                    message,
                    source: ErrorSource::FractionDenominator,
                })?;

        let numerator_range: RangeInclusive<i128> =
            source
                .numerator_range
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
