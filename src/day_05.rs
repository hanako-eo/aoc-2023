use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use aoclib::Runner;

struct Shift {
    range: Range<i64>,
    delta: i64
}

struct Mapping {
    shifts: Vec<Shift>,
    to: String
}

impl Mapping {
    fn to_shift(&self, value: i64) -> i64 {
        for shift in &self.shifts {
            if shift.range.contains(&value) {
                return value + shift.delta;
            }
        }
        value
    }
}

#[derive(Default)]
pub struct AocDay05 {
    seeds: Vec<i64>,
    maps: HashMap<String, Mapping>
}

impl AocDay05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AocDay05 {
    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self, file: File) {
        let buffer = BufReader::new(file);

        let mut lines = buffer.lines().map(|l| l.unwrap());

        self.seeds = lines.next()
            .map(|l| l.split_whitespace().skip(1).map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
            .unwrap();
        lines.next();

        while let Some(section) = lines.next() {
            let (from, to) = section.split_whitespace()
                .next()
                .unwrap()
                .split_once("-to-")
                .unwrap();

            let mut map = Mapping { to: to.to_string(), shifts: Vec::new() };

            while let Some(line) = lines.next().and_then(|l| match l.len() {
                0 => None,
                _ => Some(l)
            }) {
                let numbers = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                let (dest, src, len) = (numbers[0], numbers[1], numbers[2]);

                map.shifts.push(Shift {
                    range: Range {
                        start: src,
                        end: src + len,
                    },
                    delta: dest - src,
                });
            }

            self.maps.insert(from.to_string(), map);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.seeds.iter()
            .map(|seed| {
                let mut current_section = &"seed".to_string();
                let mut acc = *seed;
                while let Some(map) = self.maps.get(current_section) {
                    acc = map.to_shift(acc);
                    current_section = &map.to;
                }
                acc
            })
            .min()
            .unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[test]
fn day_05_part1() {
    let mut day = AocDay05::new();
    
    let file = File::open("./input/day-05-test.txt").expect("Pour avancer, il faut le fichier day-05.txt");

    day.parse(file);

    assert_eq!(day.part1(), vec!["35".to_string()]);
}

#[test]
fn day_05_part2() {
    // let mut day = AocDay05::new();
    
    // let file = File::open("./input/day-05-test.txt").expect("Pour avancer, il faut le fichier day-05.txt");

    // day.parse(file);

    // assert_eq!(day.part2(), vec!["30".to_string()]);
}
