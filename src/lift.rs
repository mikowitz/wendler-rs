#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Lift {
    Squat,
    Deadlift,
    BenchPress,
    OverheadPress,
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
