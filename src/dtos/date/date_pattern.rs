use crate::dto;
use chinese_format::gregorian;

dto! {
    pub enum DatePattern {
        Year,
        Month,
        Day,
        WeekDay,
        YearMonth,
        YearMonthDay,
        MonthDay,
        MonthDayWeekDay,
        DayWeekDay,
        YearMonthDayWeekDay,
    }
}

impl From<DatePattern> for gregorian::DatePattern {
    fn from(dto: DatePattern) -> Self {
        match dto {
            DatePattern::Year => Self::Year,
            DatePattern::Month => Self::Month,
            DatePattern::Day => Self::Day,
            DatePattern::WeekDay => Self::WeekDay,
            DatePattern::YearMonth => Self::YearMonth,
            DatePattern::YearMonthDay => Self::YearMonthDay,
            DatePattern::MonthDay => Self::MonthDay,
            DatePattern::MonthDayWeekDay => Self::MonthDayWeekDay,
            DatePattern::DayWeekDay => Self::DayWeekDay,
            DatePattern::YearMonthDayWeekDay => Self::YearMonthDayWeekDay,
        }
    }
}
