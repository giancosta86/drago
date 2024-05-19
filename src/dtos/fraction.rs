use super::{SignedRange, SignedRangeError, UnsignedRange, UnsignedRangeError};
use crate::{dto, params};
use std::ops::RangeInclusive;

dto! {
    pub struct FractionParams {
        pub denominatorRange: UnsignedRange,
        pub numeratorRange: SignedRange
    }
}

dto! {
    pub enum FractionError {
        Denominator(UnsignedRangeError),
        Numerator(SignedRangeError),
    }
}

impl TryFrom<FractionParams> for params::FractionParams {
    type Error = FractionError;

    fn try_from(dto: FractionParams) -> Result<Self, Self::Error> {
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

        Ok(Self {
            denominator_range,
            numerator_range,
        })
    }
}
