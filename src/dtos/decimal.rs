use super::{SignedRange, SignedRangeError, UnsignedRange, UnsignedRangeError};
use crate::{
    dto,
    params::{self, DigitSequenceLength},
};
use chinese_format::{self, IntegerPart};
use std::ops::RangeInclusive;

dto! {
    pub struct DecimalParams {
        pub integerRange: SignedRange,
        pub fractionalLengthRange: UnsignedRange,
    }
}

dto! {
    pub enum DecimalError {
        IntegerPart(SignedRangeError),
        FractionalLength(UnsignedRangeError)
    }
}

impl TryFrom<DecimalParams> for params::DecimalParams {
    type Error = DecimalError;

    fn try_from(dto: DecimalParams) -> Result<Self, Self::Error> {
        let integer_range: RangeInclusive<IntegerPart> = dto
            .integerRange
            .try_into()
            .map_err(DecimalError::IntegerPart)?;

        let fractional_length_range: RangeInclusive<DigitSequenceLength> = dto
            .fractionalLengthRange
            .try_into()
            .map_err(DecimalError::FractionalLength)?;

        Ok(Self {
            integer_range,
            fractional_length_range,
        })
    }
}
