use chinese_rand::gregorian::LinearTimeParams;

pub struct LinearTimeSettings {
    pub day_part: bool,
    pub include_second: bool,
}

impl From<&LinearTimeSettings> for LinearTimeParams {
    fn from(settings: &LinearTimeSettings) -> Self {
        Self {
            day_part: settings.day_part,
            include_second: settings.include_second,
        }
    }
}
