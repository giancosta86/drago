use crate::dto;
use std::ops::RangeInclusive;

dto! {
    pub struct UnsignedRange(pub u128, pub u128);
}

dto! {
    pub enum UnsignedRangeError {
        SwappedBounds(u128, u128),
        InvalidStart(u128),
        InvalidEnd(u128)
    }
}

impl TryFrom<UnsignedRange> for RangeInclusive<u128> {
    type Error = UnsignedRangeError;

    fn try_from(dto: UnsignedRange) -> Result<Self, Self::Error> {
        if dto.0 > dto.1 {
            return Err(UnsignedRangeError::SwappedBounds(dto.0, dto.1));
        }

        Ok(dto.0..=dto.1)
    }
}

macro_rules! impl_smaller_unsigned_range_conversion {
    ($target: ty) => {
        impl TryFrom<UnsignedRange> for RangeInclusive<$target> {
            type Error = UnsignedRangeError;

            fn try_from(dto: UnsignedRange) -> Result<Self, Self::Error> {
                let range_128: RangeInclusive<u128> = dto.try_into()?;

                if *range_128.start() > (<$target>::MAX as u128) {
                    return Err(UnsignedRangeError::InvalidStart(*range_128.start()));
                }

                if *range_128.end() > (<$target>::MAX as u128) {
                    return Err(UnsignedRangeError::InvalidEnd(*range_128.end()));
                }

                Ok((*range_128.start() as $target)..=(*range_128.end() as $target))
            }
        }
    };
}

impl_smaller_unsigned_range_conversion!(u8);
impl_smaller_unsigned_range_conversion!(u16);
impl_smaller_unsigned_range_conversion!(u32);
impl_smaller_unsigned_range_conversion!(u64);
impl_smaller_unsigned_range_conversion!(usize);
