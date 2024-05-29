#[allow(non_snake_case)]
mod source;

use serde::{Deserialize, Serialize};
use tsify::JsValueSerdeExt;
use wasm_bindgen::JsValue;

pub use source::ErrorSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SettingsError {
    pub message: String,
    pub source: ErrorSource,
}

impl Into<JsValue> for SettingsError {
    fn into(self) -> JsValue {
        JsValue::from_serde(&self).expect("JSON serialization should work")
    }
}
