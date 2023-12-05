use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use aoclib::Runner;

#[derive(Default)]
pub struct AocDay04 {
    cards: Vec<(Vec<i32>, Vec<i32>)>
}

impl AocDay04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AocDay04 {
    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self, file: File) {
        let buffer = BufReader::new(file);

        self.cards = buffer.lines()
            .map(|l| l.unwrap().split_once(": ").unwrap().1.to_owned())
            .filter_map(|numbers| numbers.split_once(" | ").map(|(a, b)| (a.to_owned(), b.to_owned())))
            .map(|(numbers, winning_numbers)| (
                numbers.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>(),
                winning_numbers.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            ))
            .collect::<Vec<(Vec<i32>, Vec<i32>)>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut winning_numbers_count_sum = 0;
        for (numbers, winning_numbers) in self.cards.iter() {
            // Nombre de nombre gagnant
            let mut winning_numbers_count = 0;
            for number in numbers {
                if winning_numbers.contains(number) {
                    winning_numbers_count += 1;
                }
            }
            if winning_numbers_count > 0 {
                winning_numbers_count_sum += 2i32.pow(winning_numbers_count - 1);
            }
        }

        aoclib::output(winning_numbers_count_sum)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut scalars = HashMap::<usize, u32>::new();
        for (i, (numbers, winning_numbers)) in self.cards.iter().enumerate() {
            // Nombre de nombre gagnant
            let mut winning_numbers_count = 0;
            for number in numbers {
                if winning_numbers.contains(number) {
                    winning_numbers_count += 1;
                }
            }

            let scalar = *scalars.entry(i).or_insert(1);

            for j in 0..winning_numbers_count.min(self.cards.len()) {
                scalars.entry(j + i + 1)
                    .and_modify(|next_card_scalar| *next_card_scalar += scalar)
                    .or_insert(scalar + 1);
            }
        }

        aoclib::output(scalars.values().sum::<u32>())
    }
}

#[test]
fn day_04_part1() {
    let mut day = AocDay04::new();
    
    let file = File::open("./input/day-04-test.txt").expect("Pour avancer, il faut le fichier day-04.txt");

    day.parse(file);

    assert_eq!(day.part1(), vec!["13".to_string()]);
}

#[test]
fn day_04_part2() {
    let mut day = AocDay04::new();
    
    let file = File::open("./input/day-04-test.txt").expect("Pour avancer, il faut le fichier day-04.txt");

    day.parse(file);

    assert_eq!(day.part2(), vec!["30".to_string()]);
}
