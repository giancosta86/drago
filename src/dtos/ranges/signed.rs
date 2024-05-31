use crate::{dto, SignedRangeError};
use std::ops::RangeInclusive;

dto! {
    pub struct SignedRangeDto (pub i128, pub i128);
}

impl TryInto<RangeInclusive<i128>> for SignedRangeDto {
    type Error = SignedRangeError;

    fn try_into(self) -> Result<RangeInclusive<i128>, Self::Error> {
        if self.0 > self.1 {
            return Err(SignedRangeError::SwappedBounds(self.0, self.1));
        }

        Ok(self.0..=self.1)
    }
}
