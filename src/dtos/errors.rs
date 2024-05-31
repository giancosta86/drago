use crate::dto;

dto! {
    pub enum UnsignedRangeError {
        SwappedBounds(u128, u128),
        InvalidStart(u128),
        InvalidEnd(u128)
    }
}

dto! {
    pub enum SignedRangeError {
        SwappedBounds(i128, i128),
        InvalidStart(i128),
        InvalidEnd(i128)
    }
}

dto! {
    pub enum FractionError {
        Numerator(SignedRangeError),
        Denominator(UnsignedRangeError)
    }
}

dto! {
    pub enum DecimalError {
        IntegerPart(SignedRangeError),
        FractionalLength(UnsignedRangeError)
    }
}

dto! {
    pub enum RenminbiError {
        Yuan(SignedRangeError)
    }
}

dto! {
    pub enum SettingsError {
        Integer(SignedRangeError),
        Fraction(FractionError),
        Count(UnsignedRangeError),
        DigitSequenceLength(UnsignedRangeError),
        Decimal(DecimalError),
        Renminbi(RenminbiError)
    }
}
