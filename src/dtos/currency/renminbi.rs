use super::CurrencyStyle;
use crate::{dto, UnsignedRange, UnsignedRangeError};

dto! {
    pub struct RenminbiParams {
        pub style: CurrencyStyle,
        pub yuanRange: UnsignedRange,
        pub includeDimes: bool,
        pub includeCents: bool,
    }
}

dto! {
    pub enum RenminbiError {
        Yuan(UnsignedRangeError)
    }
}

impl TryFrom<RenminbiParams> for chinese_rand::RenminbiParams {
    type Error = RenminbiError;

    fn try_from(dto: RenminbiParams) -> Result<Self, Self::Error> {
        Ok(Self {
            style: dto.style.into(),
            yuan_range: dto.yuanRange.try_into().map_err(RenminbiError::Yuan)?,
            include_dimes: dto.includeDimes,
            include_cents: dto.includeCents,
        })
    }
}
