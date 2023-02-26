use crate::accessory::Accessory;
use crate::fsl::round;
use crate::lift::Lift;

pub fn header(day: u32, lift: &Lift) -> String {
    format!("|=== Day: {} / {:?}\n", day, lift)
}

pub fn accessory_weight_entries(accessories: &[&Accessory]) -> String {
    let mut res = String::from("");
    for accessory in accessories {
        res.push_str(&format!(
            "{:?}: [underline]#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;#\n\n",
            accessory
        ));
    }
    res
}

pub fn warmups(warmup_sets: Vec<f32>) -> String {
    let mut res = String::from("\n==== Warmup\n");
    for set in warmup_sets {
        res.push_str(&format!("* [ ] 5 x {set}\n"));
    }
    res
}

pub fn fives_pro(percentages: &[f32], training_max: f32, accessory: &Accessory) -> String {
    let mut res = String::from("\n==== 5s PRO\n");
    for percentage in percentages {
        res.push_str(&format!(
            "* [ ] 5 x {}\t/ 5 x {accessory:?}\n",
            round(training_max * percentage, 4.)
        ));
    }
    res
}

pub fn fsl(
    first_set_percentage: f32,
    lift: Lift,
    training_max: f32,
    accessory: &Accessory,
) -> String {
    let mut res = format!("\n==== {lift:?} FSL\n");
    for _ in 1..=5 {
        res.push_str(&format!(
            "* [ ] 5 x {}\t/ 5 x {accessory:?}\n",
            round(training_max * first_set_percentage, 4.)
        ));
    }
    res
}

pub fn accessory_work(accessory1: &Accessory, accessory2: &Accessory) -> String {
    let mut res = String::from("\n==== Accessory\n");
    for _ in 1..=5 {
        res.push_str(&format!("* [ ] 5 x {accessory1:?}\t/ 5 x {accessory2:?}\n"));
    }
    res
}
