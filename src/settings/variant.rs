use crate::webify;

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
