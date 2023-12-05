use aoclib::{Selector, Runner};

mod day_00;
use day_00::AocDay00;

mod day_01;
use day_01::AocDay01;

mod day_02;
use day_02::AocDay02;

mod day_03;
use day_03::AocDay03;

mod day_04;
use day_04::AocDay04;

fn main() {
    run_2022(Selector::All);
}

fn run_2022(which: Selector) {
    let mut day00 = AocDay00::new();
    let mut day01 = AocDay01::new();
    let mut day02 = AocDay02::new();
    let mut day03 = AocDay03::new();
    let mut day04 = AocDay04::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day00, &mut day01, &mut day02, &mut day03,
        &mut day04,
    ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            aoclib::run_solution(days[last], last);
        }
        Selector::All => {
            let mut day_index = 0;
            for d in days {
                aoclib::run_solution(d, day_index);
                day_index += 1;
            }
        }
        Selector::One(num) => {
            aoclib::run_solution(days[num], num);
        }
    }
}


