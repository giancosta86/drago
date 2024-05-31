use crate::dto;

dto! {
    pub enum VariantDto {
        Simplified,
        Traditional
    }
}

impl Into<chinese_format::Variant> for VariantDto {
    fn into(self) -> chinese_format::Variant {
        match self {
            Self::Simplified => chinese_format::Variant::Simplified,
            Self::Traditional => chinese_format::Variant::Traditional,
        }
    }
}
