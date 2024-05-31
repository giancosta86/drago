use crate::{dto, settings::LinearTimeSettings};

dto! {
    pub struct LinearTimeSettingsDto {
        pub dayPart: bool,
        pub includeSecond: bool,
    }
}

impl From<&LinearTimeSettingsDto> for LinearTimeSettings {
    fn from(dto: &LinearTimeSettingsDto) -> Self {
        Self {
            day_part: dto.dayPart,
            include_second: dto.includeSecond,
        }
    }
}
