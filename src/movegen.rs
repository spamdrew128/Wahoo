#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Move {
    data: u16
}

impl Move {
    const TO_BITFIELD: u16    = 0b0000000000111111;
    const FROM_BITFIELD: u16  = 0b0000111111000000;
    const PROMO_BITFIELD: u16 = 0b0011000000000000;
    const FLAGS_BITFIELD: u16 = 0b1100000000000000;
}