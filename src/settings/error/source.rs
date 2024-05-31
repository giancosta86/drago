use crate::dto;

dto! {
    pub enum ErrorSource {
        Integer,
        FractionNumerator,
        FractionDenominator,
        Count,
        DigitSequence
    }
}
