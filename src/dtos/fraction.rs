use super::{FractionError, SignedRangeDto, UnsignedRangeDto, UnsignedRangeError};
use crate::{dto, settings::FractionSettings};
use std::ops::RangeInclusive;

dto! {
    pub struct FractionSettingsDto {
        pub denominatorRange: UnsignedRangeDto,
        pub numeratorRange: SignedRangeDto
    }
}

impl TryFrom<&FractionSettingsDto> for FractionSettings {
    type Error = FractionError;

    fn try_from(dto: &FractionSettingsDto) -> Result<Self, Self::Error> {
        let denominator_range: RangeInclusive<u128> = dto
            .denominatorRange
            .try_into()
            .map_err(FractionError::Denominator)?;

        if *denominator_range.start() == 0 {
            return Err(FractionError::Denominator(
                UnsignedRangeError::InvalidStart(0),
            ));
        }

        let numerator_range: RangeInclusive<i128> = dto
            .numeratorRange
            .try_into()
            .map_err(FractionError::Numerator)?;

        Ok(FractionSettings {
            denominator_range,
            numerator_range,
        })
    }
}
