use crate::dto;
use chinese_rand::gregorian;

dto! {
    pub struct LinearTimeParams {
        pub dayPart: bool,
        pub includeSecond: bool,
    }
}

impl From<LinearTimeParams> for gregorian::LinearTimeParams {
    fn from(dto: LinearTimeParams) -> Self {
        Self {
            day_part: dto.dayPart,
            include_second: dto.includeSecond,
        }
    }
}
