use crate::dto;
use chinese_format::currency::CurrencyStyle;

dto! {
    pub enum CurrencyStyleDto {
        Formal,
        Informal,
        Financial,
    }
}

impl From<CurrencyStyleDto> for CurrencyStyle {
    fn from(dto: CurrencyStyleDto) -> Self {
        match dto {
            CurrencyStyleDto::Formal => CurrencyStyle::Everyday { formal: true },
            CurrencyStyleDto::Informal => CurrencyStyle::Everyday { formal: false },
            CurrencyStyleDto::Financial => CurrencyStyle::Financial,
        }
    }
}
