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
