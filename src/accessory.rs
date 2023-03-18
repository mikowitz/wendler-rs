use std::fmt::Display;

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

impl Display for Accessory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Dips => "Dips",
            Self::ChinPulls => "Chin/Pulls",
            Self::GoodMornings => "Good Mornings",
            Self::Pushups => "Pushups",
            Self::Rows => "Rows",
            Self::AbWheel => "Ab Wheel",
            Self::TricepExtensions => "Tri. Exts.",
            Self::Curls => "Curls",
            Self::HangingLegRaises => "Leg Raises",
        };
        write!(f, "{s}")
    }
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
