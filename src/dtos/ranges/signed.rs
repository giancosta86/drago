use crate::dto;
use std::ops::RangeInclusive;

dto! {
    pub struct SignedRange (pub i128, pub i128);
}

dto! {
    pub enum SignedRangeError {
        SwappedBounds(i128, i128),
        InvalidStart(i128),
        InvalidEnd(i128)
    }
}

impl TryFrom<SignedRange> for RangeInclusive<i128> {
    type Error = SignedRangeError;

    fn try_from(dto: SignedRange) -> Result<Self, Self::Error> {
        if dto.0 > dto.1 {
            return Err(SignedRangeError::SwappedBounds(dto.0, dto.1));
        }

        Ok(dto.0..=dto.1)
    }
}
