use crate::dto;

use super::{SignedRangeDto, UnsignedRangeDto};

dto! {
    pub struct FractionSettingsDto {
        pub denominatorRange: UnsignedRangeDto,
        pub numeratorRange: SignedRangeDto
    }
}
