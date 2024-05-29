use crate::dto;
use std::ops::RangeInclusive;

dto! {
    pub struct UnsignedRangeDto(u128, u128);
}

impl TryInto<RangeInclusive<u128>> for UnsignedRangeDto {
    type Error = String;

    fn try_into(self) -> Result<RangeInclusive<u128>, Self::Error> {
        if self.0 > self.1 {
            return Err(format!("Invalid unsigned range: [{}; {}]", self.0, self.1));
        }

        Ok(self.0..=self.1)
    }
}

dto! {
    pub struct SignedRangeDto (i128, i128);
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
