use aoclib::{Selector, Runner};

mod day_00;
use day_00::AocDay00;

mod day_01;
use day_01::AocDay01;

fn main() {
    run_2022(Selector::All);
}

fn run_2022(which: Selector) {
    let mut day00 = AocDay00::new();
    let mut day01 = AocDay01::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day00, &mut day01,
    ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            aoclib::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                aoclib::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            aoclib::run_solution(*d);
        }
    }
}


