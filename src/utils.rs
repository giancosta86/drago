pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! dto {
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

#[macro_export]
macro_rules! dto_no_copy {
    ($declaration:item) => {
        #[derive(
            Debug,
            Clone,
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
