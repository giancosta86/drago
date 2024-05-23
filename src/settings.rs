use crate::webify;
use std::ops::RangeInclusive;

webify! {
    pub enum Variant {
        Simplified,
        Traditional
    }
}

impl Into<chinese_format::Variant> for Variant {
    fn into(self) -> chinese_format::Variant {
        match self {
            Self::Simplified => chinese_format::Variant::Simplified,
            Self::Traditional => chinese_format::Variant::Traditional,
        }
    }
}

webify! {
    pub struct SignedRange {
        pub min: i128,
        pub max: i128
    }
}

webify! {
    pub struct UnsignedRange {
        pub min: u128,
        pub max: u128
    }
}

webify! {
    pub struct LogogramGeneratorSettings {
        pub variant: Variant,

    }
}
