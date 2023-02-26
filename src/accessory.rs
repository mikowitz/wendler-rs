#[derive(Debug)]
pub enum Accessory {
    Dips,
    ChinPulls,
    GoodMornings,
    Pushups,
    Rows,
    AbWheel,
    TricepExtensions,
    Curls,
    HangingLegRaises,
}

impl Accessory {
    pub fn set(set_number: u32) -> (Self, Self, Self) {
        match set_number {
            0 => Self::set3(),
            1 => Self::set1(),
            2 => Self::set2(),
            3 => Self::set3(),
            _ => panic!(),
        }
    }
    fn set1() -> (Self, Self, Self) {
        (Self::Dips, Self::ChinPulls, Self::GoodMornings)
    }

    fn set2() -> (Self, Self, Self) {
        (Self::Pushups, Self::Rows, Self::AbWheel)
    }

    fn set3() -> (Self, Self, Self) {
        (Self::TricepExtensions, Self::Curls, Self::HangingLegRaises)
    }
}
