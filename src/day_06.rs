use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use aoclib::Runner;

#[derive(Default)]
pub struct AocDay06 {
    times: String,
    distances: String,
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
            .to_string();

        self.distances = lines.next()
            .unwrap()
            .split_once(":")
            .unwrap().1
            .to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut power_of_number_records = 1;
        let distances = self.distances.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        for (i, time) in self.times.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).enumerate() {
            let mut number_record = 0;
            for t in 0..=time {
                if distances[i] < t * (time - t) {
                    number_record += 1;
                }
            }
            power_of_number_records *= number_record;
        }
        
        aoclib::output(power_of_number_records)
    }
    
    fn part2(&mut self) -> Vec<String> {
        let time = self.times.split_ascii_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
        let distance = self.distances.split_ascii_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
        
        let mut number_record = 0;
        for t in 0..=time {
            if distance < t * (time - t) {
                number_record += 1;
            }
        }

        aoclib::output(number_record)
    }
}

#[test]
fn day_06_part1() {
    let mut day = AocDay06::new();
    
    let file = File::open("./input/day-06-test.txt").expect("Pour avancer, il faut le fichier day-06.txt");

    day.parse(file);

    assert_eq!(day.part1(), vec!["288".to_string()]);
}

#[test]
fn day_06_part2() {
    let mut day = AocDay06::new();
    
    let file = File::open("./input/day-06-test.txt").expect("Pour avancer, il faut le fichier day-06.txt");

    day.parse(file);

    assert_eq!(day.part2(), vec!["71503".to_string()]);
}
