use crate::dto;
use chinese_format::gregorian;

dto! {
    pub enum WeekFormat {
        XinqQi,
        Zhou,
        LiBai
    }
}

impl From<WeekFormat> for gregorian::WeekFormat {
    fn from(dto: WeekFormat) -> Self {
        match dto {
            WeekFormat::XinqQi => Self::XingQi,
            WeekFormat::Zhou => Self::Zhou,
            WeekFormat::LiBai => Self::LiBai,
        }
    }
}
