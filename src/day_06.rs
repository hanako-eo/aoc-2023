use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

use range_union_find::RangeUnionFind;
use rayon::prelude::*;

use aoclib::Runner;

#[derive(Default)]
pub struct AocDay06 {
    times: Vec<u32>,
    ditances: Vec<u32>,
}

impl AocDay06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AocDay06 {
    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self, file: File) {
        let buffer = BufReader::new(file);

        let mut lines = buffer.lines().map(|l| l.unwrap());

        self.times = lines.next()
            .unwrap()
            .split_once(":")
            .unwrap().1
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        self.ditances = lines.next()
            .unwrap()
            .split_once(":")
            .unwrap().1
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut power_of_number_records = 1;
        for (i, time) in self.times.iter().enumerate() {
            let mut number_record = 0;
            for t in 0..=*time {
                if self.ditances[i] < t * (time - t) {
                    number_record += 1;
                }
            }
            power_of_number_records *= number_record;
        }

        aoclib::output(power_of_number_records)
    }
    
    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[test]
fn day_06_part1() {
    let mut day = AocDay06::new();
    
    let file = File::open("./input/day-06-test.txt").expect("Pour avancer, il faut le fichier day-06.txt");

    day.parse(file);

    assert_eq!(day.part1(), vec!["288".to_string()]);
}

// #[test]
// fn day_06_part2() {
//     let mut day = AocDay06::new();
    
//     let file = File::open("./input/day-06-test.txt").expect("Pour avancer, il faut le fichier day-06.txt");

//     day.parse(file);

//     assert_eq!(day.part2(), vec!["46".to_string()]);
// }
