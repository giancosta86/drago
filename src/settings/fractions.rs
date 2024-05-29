use super::{SignedRangeDto, UnsignedRangeDto};
use crate::dto;

dto! {
    pub struct FractionSettingsDto {
        pub denominator_range: UnsignedRangeDto,
        pub numerator_range: SignedRangeDto
    }
}
