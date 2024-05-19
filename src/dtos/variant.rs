use crate::dto;

dto! {
    pub enum Variant {
        Simplified,
        Traditional
    }
}

impl From<Variant> for chinese_format::Variant {
    fn from(dto: Variant) -> Self {
        match dto {
            Variant::Simplified => Self::Simplified,
            Variant::Traditional => Self::Traditional,
        }
    }
}
