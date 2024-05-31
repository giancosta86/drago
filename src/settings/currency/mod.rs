use chinese_format::currency::CurrencyStyle;
use std::ops::RangeInclusive;

pub struct RenminbiSettings {
    pub style: CurrencyStyle,
    pub yuan_range: RangeInclusive<i128>,
    pub include_dimes: bool,
    pub include_cents: bool,
}
