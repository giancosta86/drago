use crate::dto;

dto! {
    pub struct UnsignedRangeDto(u128, u128);

}

dto! {
    pub struct SignedRangeDto (i128, i128);
}
