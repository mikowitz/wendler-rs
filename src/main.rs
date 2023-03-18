pub mod accessory;
pub mod adoc;
pub mod fsl;
pub mod fsl_workout;
pub mod lift;
pub mod week;
pub mod workout;

use std::{collections::HashMap, fs, process::Command};

use fsl::round;
use lift::Lift;
use week::{FslWeek, Week};
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

    let week1 = FslWeek::new(1, Lift::Deadlift, Lift::OverheadPress);
    let week2 = FslWeek::new(2, Lift::Deadlift, Lift::OverheadPress);
    let week3 = FslWeek::new(3, Lift::Deadlift, Lift::OverheadPress);
    // let week4 = FslWeek::new(4, Lift::Deadlift, Lift::OverheadPress);
    // let week5 = FslWeek::new(5, Lift::Deadlift, Lift::OverheadPress);
    // let week6 = FslWeek::new(6, Lift::Deadlift, Lift::OverheadPress);
    // let week2 = Week::new(2, Lift::Squat, Lift::BenchPress);
    // let week3 = Week::new(3, Lift::Deadlift, Lift::OverheadPress);
    // let week4 = Week::new(4, Lift::Squat, Lift::BenchPress);
    // let week5 = Week::new(5, Lift::Deadlift, Lift::OverheadPress);
    // let week6 = Week::new(6, Lift::Squat, Lift::BenchPress);

    fs::write("week1-fsl.adoc", week1.format(&training_maxes)).unwrap();
    fs::write("week2-fsl.adoc", week2.format(&training_maxes)).unwrap();
    fs::write("week3-fsl.adoc", week3.format(&training_maxes)).unwrap();
    // fs::write("week4-fsl.adoc", week4.format(&training_maxes)).unwrap();
    // fs::write("week5-fsl.adoc", week5.format(&training_maxes)).unwrap();
    // fs::write("week6-fsl.adoc", week6.format(&training_maxes)).unwrap();

    let weeks = (1..=3).map(|i| format!("week{i}")).collect::<Vec<String>>();

    for week in &weeks {
        Command::new("asciidoctor-pdf")
            .args(["-a", "pdf-theme=adoc"])
            .args(["-a", "pdf-themesdir=."])
            .arg(format!("{week}-fsl.adoc"))
            .status()
            .unwrap();
    }

    let pdfs = &weeks
        .iter()
        .map(|week| format!("{week}-fsl.pdf"))
        .collect::<Vec<String>>();

    Command::new("pdftk")
        .args(pdfs)
        .args(["output", "cycle-fsl.pdf"])
        .status()
        .unwrap();

    for week in &weeks {
        fs::remove_file(format!("{week}-fsl.adoc")).unwrap();
        fs::remove_file(format!("{week}-fsl.pdf")).unwrap();
    }
}

// Week 1
// Deadlift
// OHP
// Week 2
// Squat
// Bench
