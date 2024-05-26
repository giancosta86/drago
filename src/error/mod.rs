mod error_code;

use tsify::JsValueSerdeExt;
use wasm_bindgen::JsValue;

use crate::dto_no_copy;

pub use self::error_code::ErrorCode;

dto_no_copy! {
    struct ErrorDto {
        pub message: String,
        pub code: ErrorCode,
    }
}

impl<T> Into<Result<T, JsValue>> for ErrorCode {
    fn into(self) -> Result<T, JsValue> {
        let error_dto: ErrorDto = ErrorDto {
            code: self,
            message: self.to_string(),
        };

        let js_value = JsValue::from_serde(&error_dto).expect("JSON serialization should work");

        Err(js_value)
    }
}
