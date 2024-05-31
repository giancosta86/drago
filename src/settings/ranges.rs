use crate::{dto, errors::RangeError};
use std::ops::RangeInclusive;

dto! {
    pub struct UnsignedRangeDto(pub u128, pub u128);
}

dto! {
    pub struct SignedRangeDto (pub i128, pub i128);
}

impl TryInto<RangeInclusive<u128>> for UnsignedRangeDto {
    type Error = RangeError;

    fn try_into(self) -> Result<RangeInclusive<u128>, Self::Error> {
        if self.0 > self.1 {
            return Err(RangeError::Swapped);
        }

        Ok(self.0..=self.1)
    }
}

impl TryInto<RangeInclusive<u8>> for UnsignedRangeDto {
    type Error = String;

    fn try_into(self) -> Result<RangeInclusive<u8>, Self::Error> {
        let range_128: RangeInclusive<u128> = self.try_into()?;

        if *range_128.start() > (u8::MAX as u128) {
            return Err(format!("Invalid u8 start value: {}", range_128.start()));
        }

        if *range_128.end() > (u8::MAX as u128) {
            return Err(format!("Invalid u8 end value: {}", range_128.end()));
        }

        Ok((*range_128.start() as u8)..=(*range_128.end() as u8))
    }
}

impl TryInto<RangeInclusive<i128>> for SignedRangeDto {
    type Error = String;

    fn try_into(self) -> Result<RangeInclusive<i128>, Self::Error> {
        if self.0 > self.1 {
            return Err(format!("Invalid signed range: [{}; {}]", self.0, self.1));
        }

        Ok(self.0..=self.1)
    }
}
