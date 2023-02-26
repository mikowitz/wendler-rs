use crate::accessory::Accessory;
use crate::lift::Lift;
use std::collections::HashMap;

const WEEK_1_2_PERCENTAGES: (f32, f32, f32) = (0.65, 0.75, 0.85);
const WEEK_3_4_PERCENTAGES: (f32, f32, f32) = (0.70, 0.80, 0.90);
const WEEK_5_6_PERCENTAGES: (f32, f32, f32) = (0.75, 0.85, 0.95);

pub fn print_lifts_for_week(
    week: u32,
    lift: Lift,
    training_maxes: &HashMap<&Lift, f32>,
    accessory_set: u32,
) -> () {
    let (p1, p2, p3) = percentages_for_week(week);
    let (push, pull, core) = Accessory::set(accessory_set);

    let (main_superset, core_superset) = match lift {
        Lift::Squat | Lift::Deadlift => (push, pull),
        _ => (pull, push),
    };

    let secondary_lift = lift.secondary_lift();

    let primary_training_max = *training_maxes.get(&lift).unwrap();
    let secondary_training_max = *training_maxes.get(&secondary_lift).unwrap();

    let warmup_sets = warmup_sets(
        p1 * primary_training_max,
        lift.base_weight(),
        primary_training_max,
    );

    println!("# {lift:?}");
    println!("# Warmup");
    for set in warmup_sets {
        println!("* [ ] 5 x {set}");
    }

    println!("# 5s PRO");
    for percentage in vec![p1, p2, p3] {
        println!(
            "* [ ] 5 x {} || 5 x {main_superset:?}",
            round(primary_training_max * percentage, 4.)
        );
    }
    println!("# {secondary_lift:?} FSL");
    for _ in 1..=5 {
        println!(
            "* [ ] 5 x {} || 5 x {main_superset:?}",
            round(secondary_training_max * p1, 4.)
        );
    }

    println!("# Accessory");
    for _ in 1..=5 {
        println!("* [ ] 5 x {core_superset:?} || 5 x {core:?}");
    }
}

pub fn percentages_for_week(week: u32) -> (f32, f32, f32) {
    match week {
        1 | 2 => WEEK_1_2_PERCENTAGES,
        3 | 4 => WEEK_3_4_PERCENTAGES,
        5 | 6 => WEEK_5_6_PERCENTAGES,
        _ => panic!(),
    }
}

pub fn round(weight: f32, granularity: f32) -> f32 {
    (weight * granularity / 10.).round() as f32 * 10. / granularity
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
