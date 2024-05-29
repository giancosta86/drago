mod source;

use crate::dto_no_copy;
use tsify::JsValueSerdeExt;
use wasm_bindgen::JsValue;

pub use source::ErrorSource;

dto_no_copy! {
    pub struct SettingsError {
        pub message: String,
        pub source: ErrorSource,
    }
}

impl Into<JsValue> for SettingsError {
    fn into(self) -> JsValue {
        JsValue::from_serde(&self).expect("JSON serialization should work")
    }
}
