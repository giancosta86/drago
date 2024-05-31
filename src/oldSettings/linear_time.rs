use super::LinearTimeSettingsDto;
use chinese_rand::gregorian::LinearTimeParams;

pub struct LinearTimeSettings {
    pub day_part: bool,
    pub include_second: bool,
}

impl From<LinearTimeSettingsDto> for LinearTimeSettings {
    fn from(dto: LinearTimeSettingsDto) -> Self {
        Self {
            day_part: dto.dayPart,
            include_second: dto.includeSecond,
        }
    }
}

impl Into<LinearTimeParams> for &LinearTimeSettings {
    fn into(self) -> LinearTimeParams {
        LinearTimeParams {
            day_part: self.day_part,
            include_second: self.include_second,
        }
    }
}
