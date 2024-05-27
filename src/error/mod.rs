mod error_code;

use tsify::JsValueSerdeExt;
use wasm_bindgen::JsValue;

use crate::dto_no_copy;

pub use self::error_code::ErrorSource;

dto_no_copy! {
    pub struct GeneratorError {
        pub message: String,
        pub source: ErrorSource,
    }
}

impl Into<JsValue> for GeneratorError {
    fn into(self) -> JsValue {
        JsValue::from_serde(&self).expect("JSON serialization should work")
    }
}
