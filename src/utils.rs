pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! webify {
    ($declaration:item) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            tsify::Tsify,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[tsify(into_wasm_abi, from_wasm_abi)]
        $declaration
    };
}
