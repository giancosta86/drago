use crate::dto;
use chinese_format::currency;

dto! {
    pub enum CurrencyStyle {
        Formal,
        Informal,
        Financial,
    }
}

impl From<CurrencyStyle> for currency::CurrencyStyle {
    fn from(dto: CurrencyStyle) -> Self {
        match dto {
            CurrencyStyle::Formal => Self::Everyday { formal: true },
            CurrencyStyle::Informal => Self::Everyday { formal: false },
            CurrencyStyle::Financial => Self::Financial,
        }
    }
}
