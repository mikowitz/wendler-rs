use std::collections::HashMap;

use crate::fsl_workout::FslWorkout;
use crate::lift::Lift;
use crate::workout::Workout;

#[derive(Debug)]
pub struct Week {
    week: u32,
    day1: Workout,
    day2: Workout,
}

#[derive(Debug)]
pub struct FslWeek {
    week: u32,
    day1: FslWorkout,
    day2: FslWorkout,
}

impl Week {
    pub fn new(week: u32, day_1_lift: Lift, day_2_lift: Lift) -> Self {
        let day1 = Workout::new(week, 1, day_1_lift);
        let day2 = Workout::new(week, 2, day_2_lift);
        Self { week, day1, day2 }
    }

    pub fn format(&self, training_maxes: &HashMap<&Lift, f32>) -> String {
        format!(
            r#"
=== Week {}

[cols="2a,2a"]
|===
{}
{}
|===

        "#,
            self.week,
            self.day1.format(training_maxes),
            self.day2.format(training_maxes)
        )
    }
}

impl FslWeek {
    pub fn new(week: u32, day_1_lower: Lift, day_1_upper: Lift) -> Self {
        let day1 = FslWorkout::new(week, 1, day_1_lower, day_1_upper);
        let day2 = FslWorkout::new(
            week,
            2,
            day_1_lower.secondary_lift(),
            day_1_upper.secondary_lift(),
        );
        Self { week, day1, day2 }
    }

    pub fn format(&self, training_maxes: &HashMap<&Lift, f32>) -> String {
        format!(
            r#"
=== Week {}

[cols="2a,2a"]
|===
{}
{}
|===

        "#,
            self.week,
            self.day1.format(training_maxes),
            self.day2.format(training_maxes)
        )
    }
}
