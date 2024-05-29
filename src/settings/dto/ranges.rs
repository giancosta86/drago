use crate::dto;

dto! {
    pub struct UnsignedRangeDto(pub u128, pub u128);
}

dto! {
    pub struct SignedRangeDto (pub i128, pub i128);
}
