use crate::dto;
use chinese_rand::gregorian::LinearTimeParams;

dto! {
    pub struct LinearTimeSettingsDto {
        pub dayPart: bool,
        pub includeSecond: bool,
    }
}

impl From<LinearTimeSettingsDto> for LinearTimeParams {
    fn from(dto: LinearTimeSettingsDto) -> Self {
        Self {
            day_part: dto.dayPart,
            include_second: dto.includeSecond,
        }
    }
}
