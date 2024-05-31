use chinese_format::currency::CurrencyStyle;

use super::UnsignedRangeDto;

pub enum CurrencyStyleSettingsDto {
    Formal,
    Informal,
    Financial,
}

impl Into<CurrencyStyle> for CurrencyStyleSettingsDto {
    fn into(self) -> CurrencyStyle {
        match self {
            CurrencyStyleSettingsDto::Formal => CurrencyStyle::Everyday { formal: true },
            CurrencyStyleSettingsDto::Informal => CurrencyStyle::Everyday { formal: false },
            CurrencyStyleSettingsDto::Financial => CurrencyStyle::Financial,
        }
    }
}

pub struct RenminbiSettingsDto {
    pub style: CurrencyStyleSettingsDto,
    pub yuan_range: UnsignedRangeDto,
    pub include_dimes: bool,
    pub include_cents: bool,
}
