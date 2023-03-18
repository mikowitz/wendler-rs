use std::collections::HashMap;

use crate::accessory::Accessory;
use crate::adoc;
use crate::fsl::round;
use crate::lift::Lift;

const WEEK_1_2_PERCENTAGES: (f32, f32, f32) = (0.65, 0.75, 0.85);
const WEEK_3_4_PERCENTAGES: (f32, f32, f32) = (0.70, 0.80, 0.90);
const WEEK_5_6_PERCENTAGES: (f32, f32, f32) = (0.75, 0.85, 0.95);

#[derive(Debug)]
pub struct FslWorkout {
    week: u32,
    day: u32,
    lower_body: Lift,
    upper_body: Lift,
    accessory_set: (Accessory, Accessory, Accessory),
}

impl FslWorkout {
    pub fn new(week: u32, day: u32, lower_body: Lift, upper_body: Lift) -> Self {
        let accessory_set_number = if day == 1 {
            (week * 2 - 1) % 3
        } else {
            (week * 2) % 3
        };
        let accessory_set = Accessory::set(accessory_set_number);
        Self {
            week,
            day,
            lower_body,
            upper_body,
            accessory_set,
        }
    }

    pub fn format(&self, training_maxes: &HashMap<&Lift, f32>) -> String {
        let (p1, p2, p3) = percentages_for_week(self.week);
        let (push, pull, core) = &self.accessory_set;

        let lower_training_max = *training_maxes.get(&self.lower_body).unwrap();
        let upper_training_max = *training_maxes.get(&self.upper_body).unwrap();

        let lower_warmup_sets = warmup_sets(
            p1 * lower_training_max,
            self.lower_body.base_weight(),
            lower_training_max,
        );

        let upper_warmup_sets = warmup_sets(
            p1 * upper_training_max,
            self.upper_body.base_weight(),
            upper_training_max,
        );

        let mut res = adoc::fsl_header(self.day, &self.lower_body, &self.upper_body);
        res.push_str(&adoc::accessory_weight_entries(&[push, pull, core]));

        res.push_str(&adoc::warmups(self.lower_body, lower_warmup_sets));
        res.push_str(&adoc::fives_pro(
            self.lower_body,
            &[p1, p2, p3],
            lower_training_max,
            push,
        ));
        res.push_str(&adoc::fsl(
            p1,
            self.lower_body.clone(),
            lower_training_max,
            push,
        ));

        res.push_str(&adoc::warmups(self.upper_body, upper_warmup_sets));
        res.push_str(&adoc::fives_pro(
            self.upper_body,
            &[p1, p2, p3],
            upper_training_max,
            pull,
        ));
        res.push_str(&adoc::fsl(
            p1,
            self.upper_body.clone(),
            upper_training_max,
            pull,
        ));

        res.push_str(&adoc::core_only_accessory_work(core));

        res
    }
}

fn percentages_for_week(week: u32) -> (f32, f32, f32) {
    match week {
        1 => WEEK_1_2_PERCENTAGES,
        2 => WEEK_3_4_PERCENTAGES,
        3 => WEEK_5_6_PERCENTAGES,
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
