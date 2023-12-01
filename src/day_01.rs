use std::io::{BufReader, BufRead};
use std::fs::File;

use aoclib::Runner;

#[derive(Default)]
pub struct AocDay01 {
    pub(crate) calibrations_values: Vec<u32>,
}

impl AocDay01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AocDay01 {
    fn day(&self) -> usize { 1 }

    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self) {
        let file = File::open("./input/day-01.txt").expect("Pour avancer, il faudrait mettre le fichier dayy-01");
        let buffer = BufReader::new(file);

        for line in buffer.lines().map(|l| l.unwrap()) {
            let mut first_digit: Option<u32> = None;
            let mut last_digit: Option<u32> = None;

            for c in line.chars() {
                if c.is_ascii_digit() {
                    if first_digit.is_none() {
                        first_digit = c.to_digit(10);
                    }
                    last_digit = c.to_digit(10);
                }
            }

            let first_digit = first_digit.unwrap();
            let last_digit = last_digit.unwrap();
            self.calibrations_values.push(first_digit * 10 + last_digit)
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.calibrations_values.iter().sum::<u32>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("")
    }
}

#[test]
fn day_01_file_reading() {
    let mut day = AocDay01::default();
    day.parse();

    assert_eq!(day.calibrations_values.get(0..10).unwrap(), vec![88, 23, 33, 45, 22, 53, 55, 82, 14, 69]);
}

#[test]
fn day_01_part1() {
    /*
    Input:
    1abc2        | 1,2 => 12
    pqr3stu8vwx  | 3,8 => 38
    a1b2c3d4e5f  | 1(,2,3,4),5 => 15
    treb7uchet   | 7 => 77

    Result = 12 + 38 + 15 + 77 = 142
    */

    let mut day = AocDay01 {
        calibrations_values: vec![12, 38, 15, 77]
    };

    assert_eq!(day.part1(), vec!["142".to_string()]);
}

#[test]
fn day_01_part2() {
    // let mut day = AocDay01 {
    //     lines: Default::default(),
    //     matrixs: vec![
    //         vec!["..C...s..".to_string()],
    //         vec!["..C......".to_string(),
    //              "....s....".to_string()],
    //         vec!["........C".to_string(),
    //              ".........".to_string(),
    //              ".........".to_string()],
    //         vec![".C.......".to_string(),
    //              ".........".to_string(),
    //              "......s..".to_string()],
    //         vec!["........C".to_string(),
    //              "s........".to_string(),
    //              ".........".to_string()],
    //         vec![".........".to_string(),
    //              "...s.....".to_string(),
    //              ".........".to_string()],
    //         vec!["........C".to_string(),
    //              "........s".to_string(),
    //              ".........".to_string()],
    //         vec![".........".to_string(),
    //              ".........".to_string(),
    //              "...C.s...".to_string()],
    //     ],
    // };

    // assert_eq!(day.part2(), vec!["43079012".to_string()]);
}
