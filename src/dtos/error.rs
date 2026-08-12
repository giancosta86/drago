use super::{
    DateError, DecimalError, FractionError, RenminbiError, SignedRangeError, UnsignedRangeError,
};
use crate::dto;

dto! {
    pub enum RandomParamsError {
        EmptyRandomParams,
        Integer(SignedRangeError),
        Fraction(FractionError),
        Count(UnsignedRangeError),
        DigitSequenceLength(UnsignedRangeError),
        Decimal(DecimalError),
        Renminbi(RenminbiError),
        Date(DateError)
    }
}
