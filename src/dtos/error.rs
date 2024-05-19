use super::{
    DateError, DecimalError, FractionError, RenminbiError, SignedRangeError, UnsignedRangeError,
};
use crate::dto;
use tsify::JsValueSerdeExt;
use wasm_bindgen::JsValue;

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

impl From<RandomParamsError> for JsValue {
    fn from(error: RandomParamsError) -> Self {
        JsValue::from_serde(&error)
            .expect("Serialization to JsValue for errors is expected to work")
    }
}
