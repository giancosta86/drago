use crate::webify;

webify! {
    pub struct SignedRange {
        pub min: i128,
        pub max: i128,
    }
}

webify! {
    pub struct UnsignedRange {
        pub min: u128,
        pub max: u128
    }
}
