use std::io::{BufReader, BufRead};
use std::fs::File;

use aoclib::Runner;

#[derive(Default)]
pub struct AocDay01 {
    lines: Vec<String>,
}

impl AocDay01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AocDay01 {
    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self) {
        let file = File::open("./input/day-01.txt")
            .expect("Pour avancer, il faut le fichier day-01.txt");
        let buffer = BufReader::new(file);

        self.lines = buffer.lines().map(|l| l.unwrap()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut calibration_value = 0;
        for line in self.lines.iter() {
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

            calibration_value += first_digit.unwrap() * 10 + last_digit.unwrap();
        }

        aoclib::output(calibration_value)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut calibration_value = 0;
        let numbers = ["one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];

        // NOTE: cette version de la boucle peut très certainement être améliorée, mais 
        // fait une boucle et non plus 2 pour trouver le maximum et le minimum
        // elle peut potentiellement être rendue plus rapide avec l'algo Aho-Corasick
        for line in self.lines.iter() {
            // trouve l'ensemble des chiffres dans le line
            let mut iterator = numbers.iter()
                .enumerate()
                .flat_map(|(i, number)| line
                    .match_indices(number)
                    .map(move |(str_i, _)| (i, str_i))
                );

            let mut first_digit_pos: (usize, usize) = iterator.next().unwrap();
            let mut last_digit_pos: (usize, usize) = first_digit_pos.clone();

            for pos in iterator {
                if pos.1 < first_digit_pos.1 {
                    first_digit_pos = pos;
                } else if pos.1 > last_digit_pos.1 {
                    last_digit_pos = pos;
                }
            }

            // transforme la position dans le tableau en ca valeur, le calcul est équivalent à $index / 2 + 1$ 
            let first_digit = (first_digit_pos.0 >> 1) + 1;
            let last_digit = (last_digit_pos.0 >> 1) + 1;

            calibration_value += first_digit * 10 + last_digit;
        }

        aoclib::output(calibration_value)
    }
}

#[test]
fn day_01_file_reading() {
    let mut day = AocDay01::default();
    day.parse();

    assert_eq!(day.lines.get(0..10).unwrap(), vec![
        "five8b".to_string(),
        "2733vmmpknvgr".to_string(),
        "3oneeighttwo".to_string(),
        "twofourfive485".to_string(),
        "2fourghsixptk".to_string(),
        "5fivezgfgcxbf3five".to_string(),
        "eighthtkk5".to_string(),
        "qjqpnfs812sevensbjlkzrzczdmsr".to_string(),
        "cpxtthree14".to_string(),
        "pljnzhchrrqvkncfnfive6four7dzqkfslm9".to_string(),
    ]);
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
        lines: vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ]
    };

    assert_eq!(day.part1(), vec!["142".to_string()]);
}

#[test]
fn day_01_part2() {
    let mut day = AocDay01 {
        lines: vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
            "twone".to_string(),
            "one1twoninehm".to_string(),
        ]
    };

    assert_eq!(day.part2(), vec!["321".to_string()]);
}
