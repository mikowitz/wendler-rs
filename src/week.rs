use std::collections::HashMap;

use crate::lift::Lift;
use crate::workout::Workout;

#[derive(Debug)]
pub struct Week {
    week: u32,
    day1: Workout,
    day2: Workout,
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
