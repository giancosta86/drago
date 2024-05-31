use super::{DecimalSettingsDto, ErrorSource, SettingsError};
use chinese_format::IntegerPart;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecimalSettings {
    pub integer_range: RangeInclusive<IntegerPart>,
    pub fractional_length_range: RangeInclusive<u8>,
}

impl TryFrom<&DecimalSettingsDto> for DecimalSettings {
    type Error = SettingsError;

    fn try_from(dto: &DecimalSettingsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            integer_range: dto
                .integerRange
                .try_into()
                .map_err(|message| SettingsError {
                    message,
                    source: ErrorSource::DecimalIntegerPart,
                })?,
            fractional_length_range: dto.fractionalLengthRange.try_into().map_err(|message| {
                SettingsError {
                    message,
                    source: ErrorSource::DecimalFractionalPart,
                }
            })?,
        })
    }
}
