use super::{DecimalError, SignedRangeDto, UnsignedRangeDto};
use crate::{dto, settings::DecimalSettings};
use chinese_format::IntegerPart;
use std::ops::RangeInclusive;

dto! {
    pub struct DecimalSettingsDto {
        pub integerRange: SignedRangeDto,
        pub fractionalLengthRange: UnsignedRangeDto,
    }
}

impl TryFrom<DecimalSettingsDto> for DecimalSettings {
    type Error = DecimalError;

    fn try_from(dto: DecimalSettingsDto) -> Result<Self, Self::Error> {
        let integer_range: RangeInclusive<IntegerPart> = dto
            .integerRange
            .try_into()
            .map_err(DecimalError::IntegerPart)?;

        let fractional_length_range: RangeInclusive<u8> = dto
            .fractionalLengthRange
            .try_into()
            .map_err(DecimalError::FractionalLength)?;

        Ok(DecimalSettings {
            integer_range,
            fractional_length_range,
        })
    }
}
