use super::{SignedRangeDto, UnsignedRangeDto};
use crate::dto;

dto! {
    pub struct DecimalSettingsDto {
        pub integerRange: SignedRangeDto,
        pub fractionalLengthRange: UnsignedRangeDto,
    }
}
