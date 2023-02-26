use std::collections::HashMap;

use crate::accessory::Accessory;
use crate::fsl::round;
use crate::lift::Lift;

const WEEK_1_2_PERCENTAGES: (f32, f32, f32) = (0.65, 0.75, 0.85);
const WEEK_3_4_PERCENTAGES: (f32, f32, f32) = (0.70, 0.80, 0.90);
const WEEK_5_6_PERCENTAGES: (f32, f32, f32) = (0.75, 0.85, 0.95);

#[derive(Debug)]
pub struct Workout {
    week: u32,
    day: u32,
    main_lift: Lift,
    accessory_set: (Accessory, Accessory, Accessory),
}

impl Workout {
    pub fn new(week: u32, day: u32, main_lift: Lift) -> Self {
        let accessory_set_number = if day == 1 {
            (week * 2 - 1) % 3
        } else {
            (week * 2) % 3
        };
        let accessory_set = Accessory::set(accessory_set_number);
        Self {
            week,
            day,
            main_lift,
            accessory_set,
        }
    }

    pub fn format(&self, training_maxes: &HashMap<&Lift, f32>) -> String {
        let (p1, p2, p3) = percentages_for_week(self.week);
        let (push, pull, core) = &self.accessory_set;

        let (main_superset, core_superset) = match self.main_lift {
            Lift::Squat | Lift::Deadlift => (push, pull),
            _ => (pull, push),
        };

        let secondary_lift = self.main_lift.secondary_lift();

        let primary_training_max = *training_maxes.get(&self.main_lift).unwrap();
        let secondary_training_max = *training_maxes.get(&secondary_lift).unwrap();

        let warmup_sets = warmup_sets(
            p1 * primary_training_max,
            self.main_lift.base_weight(),
            primary_training_max,
        );
        let mut res = String::from(&format!("|=== Day: {} / {:?}\n", self.day, self.main_lift));
        res.push_str(&format!(
            "{:?}: [underline]#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;#\n\n",
            main_superset
        ));
        res.push_str(&format!(
            "{:?}: [underline]#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;#\n\n",
            core_superset
        ));
        res.push_str(&format!(
            "{:?}: [underline]#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;#\n\n",
            core
        ));
        res.push_str("\n==== Warmup\n");
        for set in warmup_sets {
            res.push_str(&format!("* [ ] 5 x {set}\n"));
        }
        res.push_str("\n==== 5s PRO\n");
        for percentage in &[p1, p2, p3] {
            res.push_str(&format!(
                "* [ ] 5 x {}\t/ 5 x {main_superset:?}\n",
                round(primary_training_max * percentage, 4.)
            ));
        }

        res.push_str(&format!("\n==== {secondary_lift:?} FSL\n"));
        for _ in 1..=5 {
            res.push_str(&format!(
                "* [ ] 5 x {}\t/ 5 x {main_superset:?}\n",
                round(secondary_training_max * p1, 4.)
            ));
        }
        res.push_str("\n==== Accessory\n");
        for _ in 1..=5 {
            res.push_str(&format!("* [ ] 5 x {core_superset:?}\t/ 5 x {core:?}\n"));
        }

        res
    }
}

fn percentages_for_week(week: u32) -> (f32, f32, f32) {
    match week {
        1 | 2 => WEEK_1_2_PERCENTAGES,
        3 | 4 => WEEK_3_4_PERCENTAGES,
        5 | 6 => WEEK_5_6_PERCENTAGES,
        _ => panic!(),
    }
}

fn warmup_sets(first_set: f32, base_weight: f32, training_max: f32) -> Vec<f32> {
    let ten_pct = training_max * 0.1;
    let mut warmups: Vec<f32> = vec![];
    let mut next_warmup = first_set - ten_pct;

    while next_warmup > base_weight {
        warmups.push(next_warmup);
        next_warmup -= ten_pct;
    }
    warmups.push(base_weight);
    warmups.reverse();
    warmups = warmups.iter().map(|w| round(*w, 2.)).collect::<Vec<f32>>();
    warmups.dedup();
    warmups
}
