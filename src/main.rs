pub mod accessory;
pub mod fsl;
pub mod lift;
pub mod week;
pub mod workout;

use std::{collections::HashMap, fs, process::Command};

use fsl::round;
use lift::Lift;
use week::Week;
// use workout::Workout;

fn main() {
    let one_rep_maxes = HashMap::from([
        (Lift::Squat, 225.),
        (Lift::Deadlift, 260.),
        (Lift::BenchPress, 160.),
        (Lift::OverheadPress, 105.),
    ]);

    let training_maxes: HashMap<&Lift, f32> = one_rep_maxes
        .iter()
        .map(|(k, v)| (k, round(v * 0.85, 4.)))
        .collect();

    let week1 = Week::new(1, Lift::Deadlift, Lift::OverheadPress);
    let week2 = Week::new(2, Lift::Squat, Lift::BenchPress);
    let week3 = Week::new(3, Lift::Deadlift, Lift::OverheadPress);
    let week4 = Week::new(4, Lift::Squat, Lift::BenchPress);
    let week5 = Week::new(5, Lift::Deadlift, Lift::OverheadPress);
    let week6 = Week::new(6, Lift::Squat, Lift::BenchPress);

    fs::write("week1.adoc", week1.format(&training_maxes)).unwrap();
    fs::write("week2.adoc", week2.format(&training_maxes)).unwrap();
    fs::write("week3.adoc", week3.format(&training_maxes)).unwrap();
    fs::write("week4.adoc", week4.format(&training_maxes)).unwrap();
    fs::write("week5.adoc", week5.format(&training_maxes)).unwrap();
    fs::write("week6.adoc", week6.format(&training_maxes)).unwrap();

    let weeks = (1..=6).map(|i| format!("week{i}")).collect::<Vec<String>>();

    for week in &weeks {
        Command::new("asciidoctor-pdf")
            .arg(format!("{week}.adoc"))
            .status()
            .unwrap();
    }

    let pdfs = &weeks
        .iter()
        .map(|week| format!("{week}.pdf"))
        .collect::<Vec<String>>();

    Command::new("pdftk")
        .args(pdfs)
        .args(["output", "cycle.pdf"])
        .status()
        .unwrap();

    for week in &weeks {
        fs::remove_file(format!("{week}.adoc")).unwrap();
        fs::remove_file(format!("{week}.pdf")).unwrap();
    }
}

// Week 1
// Deadlift
// OHP
// Week 2
// Squat
// Bench
