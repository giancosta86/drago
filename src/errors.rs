use crate::dto;

dto! {
    pub enum UnsignedRangeError {
        SwappedBounds,
        InvalidMin(u128),
        InvalidMax(u128)
    }
}

dto! {
    pub enum SignedRangeError {
        SwappedBounds,
        InvalidMin(i128),
        InvalidMax(i128)
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
        DecimalPartLength(UnsignedRangeError)
    }
}

dto! {
    pub enum Error {
        IntegerRange(SignedRangeError),
        Fraction(FractionError),
        CountRange(UnsignedRangeError),
        DigitSequenceLength(UnsignedRangeError),
        Decimal(DecimalError)
    }
}
