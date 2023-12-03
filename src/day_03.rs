use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use aoclib::Runner;
use aoclib::boxmut::BoxMut;

type Position = (usize, usize);

struct Number {
    value: u32,
    used: bool
}

struct Piece {
    symbol: char,
    position: Position,
    numbers: Vec<u32>,
}

#[derive(Default)]
pub struct AocDay03 {
    pieces: Vec<Piece>,
    numbers: HashMap<Position, BoxMut<Number>>
}

impl AocDay03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn parse_schema(&mut self, buffer: BufReader<File>) {
        for (y, line) in buffer.lines().map(|l| l.unwrap()).enumerate() {
            let mut chars_buffer = line.char_indices();
            let mut current_char = chars_buffer.next();
            while let Some((x, c)) = current_char {
                match c {
                    '.' => (),
                    n if n.is_ascii_digit() => {
                        let mut number_token = String::new();
                        while let Some((_, c)) = current_char {
                            if !c.is_ascii_digit() {
                                break;
                            }
                            number_token.push(c);
                            current_char = chars_buffer.next();
                        }
                        let number = BoxMut::new(Number {
                            value: number_token.parse::<u32>().unwrap(),
                            used: false
                        });
                        self.numbers.insert((x, y), number.clone());
                        self.numbers.insert((x + number_token.len() - 1, y), number);
                        continue;
                    }
                    symbol => self.pieces.push(Piece { symbol, position: (x, y), numbers: Vec::new() })
                }
                current_char = chars_buffer.next();
            }
        }

        for piece in self.pieces.iter_mut() {
            let (x, y) = piece.position;
            for delta_x in -1isize..=1 {
                for delta_y in -1isize..=1 {
                    if let Some(number) = self.numbers.get_mut(&((x as isize).saturating_add(delta_x) as usize, (y as isize).saturating_add(delta_y) as usize)) {
                        let number = number.get_mut();
                        if !number.used {
                            piece.numbers.push(number.value);
                            number.used = true;
                        }
                    }
                }
            }
        }
    }
}

impl Runner for AocDay03 {
    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self) {
        let file =
            File::open("./input/day-03.txt").expect("Pour avancer, il faut le fichier day-03.txt");
        let buffer = BufReader::new(file);

        self.parse_schema(buffer);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.pieces.iter().map(|p| p.numbers.iter().sum::<u32>()).sum::<u32>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.pieces.iter().filter_map(|p| match p.symbol {
            '*' if p.numbers.len() > 1 => Some(&p.numbers),
            _ => None
        }).map(|numbers| numbers.iter().product::<u32>()).sum::<u32>())
    }
}

#[test]
fn day_03_part1() {
    let mut day = AocDay03::new();
    
    let file = File::open("./input/day-03-test.txt").expect("Pour avancer, il faut le fichier day-03.txt");
    let buffer = BufReader::new(file);

    day.parse_schema(buffer);

    assert_eq!(day.part1(), vec!["4361".to_string()]);
}

#[test]
fn day_03_part2() {
    let mut day = AocDay03::new();
    
    let file = File::open("./input/day-03-test.txt").expect("Pour avancer, il faut le fichier day-03.txt");
    let buffer = BufReader::new(file);
    
    day.parse_schema(buffer);
    
    assert_eq!(day.part2(), vec!["467835".to_string()]);
}
