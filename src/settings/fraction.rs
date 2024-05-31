use super::{SignedRangeDto, UnsignedRangeDto};
use crate::dto;

dto! {
    pub struct FractionSettingsDto {
        pub denominatorRange: UnsignedRangeDto,
        pub numeratorRange: SignedRangeDto
    }
}
