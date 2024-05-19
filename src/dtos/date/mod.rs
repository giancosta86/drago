mod date_pattern;
mod week_format;

pub use date_pattern::*;
pub use week_format::*;

use super::{UnsignedRange, UnsignedRangeError};
use crate::dto;
use chinese_rand::gregorian;

dto! {
    pub struct DateParams {
        pub pattern: DatePattern,

        #[tsify(optional)]
        pub yearRange: Option<UnsignedRange>,

        pub formal: bool,

        #[tsify(optional)]
        pub weekFormat: Option<WeekFormat>,
    }
}

dto! {
    pub enum DateError {
        Year(UnsignedRangeError)
    }
}

impl TryFrom<DateParams> for gregorian::DateParams {
    type Error = DateError;

    fn try_from(dto: DateParams) -> Result<Self, Self::Error> {
        let year_range = dto
            .yearRange
            .map(|range_dto| range_dto.try_into().map_err(DateError::Year))
            .transpose()?;

        Ok(Self {
            pattern: dto.pattern.into(),
            year_range,
            formal: dto.formal,
            week_format: dto.weekFormat.map(|week_format_dto| week_format_dto.into()),
        })
    }
}
