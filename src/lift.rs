use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lift {
    Squat,
    Deadlift,
    BenchPress,
    OverheadPress,
}

impl Display for Lift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Squat => "Squat",
            Self::Deadlift => "DL",
            Self::BenchPress => "Bench",
            Self::OverheadPress => "OH Press",
        };
        write!(f, "{s}")
    }
}

impl Lift {
    pub fn secondary_lift(&self) -> Self {
        match self {
            Self::Squat => Self::Deadlift,
            Self::Deadlift => Self::Squat,
            Self::BenchPress => Self::OverheadPress,
            Self::OverheadPress => Self::BenchPress,
        }
    }

    pub fn base_weight(&self) -> f32 {
        match self {
            Self::Squat | Self::Deadlift => 95.,
            _ => 45.,
        }
    }
}
