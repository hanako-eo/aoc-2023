use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

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
    number: u32,
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
                        self.numbers.insert((x, y), number);
                        self.numbers.insert((x + number_token.len() - 1, y), number);
                        continue;
                    }
                    symbol => self.pieces.push(Piece { symbol, position: (x, y), number: 0 })
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
                            piece.number += number.value;
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
        aoclib::output(self.pieces.iter().map(|p| p.number).sum::<u32>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("")
    }
}

#[test]
fn day_03_file_reading() {
    // let mut day = AocDay03::default();
    // day.parse();

    // assert_eq!(day.games.get(0..3).unwrap(), vec![
    //         vec![
    //             Round { green: 1, blue: 7, red: 9 },
    //             Round { green: 8, blue: 0, red: 0 },
    //             Round { green: 10, blue: 5, red: 3 },
    //             Round { green: 1, blue: 11, red: 5  }
    //         ],
    //         vec![
    //             Round { green: 7, blue: 3, red: 0 },
    //             Round { green: 4, blue: 20, red: 0 },
    //             Round { green: 2, blue: 13, red: 6  }
    //         ],
    //         vec![
    //             Round { green: 1, blue: 11, red: 3 },
    //             Round { green: 3, blue: 9, red: 15 },
    //             Round { green: 4, blue: 11, red: 4 },
    //             Round { green: 2, blue: 14, red: 1 },
    //             Round { green: 4, blue: 18, red: 10  }
    //         ]
    //     ]
    // );
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
}
