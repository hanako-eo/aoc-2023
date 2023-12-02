use std::fs::File;
use std::io::{BufRead, BufReader};

use aoclib::Runner;

#[derive(Debug, Default, PartialEq, Eq)]
struct Round {
    green: u32,
    blue: u32,
    red: u32,
}

#[derive(Default)]
pub struct AocDay02 {
    reference_bag: Round,
    games: Vec<Vec<Round>>,
}

impl AocDay02 {
    pub fn new() -> Self {
        Self {
            reference_bag: Round { green: 13, blue: 14, red: 12 },
            ..Default::default()
        }
    }
}

impl Runner for AocDay02 {
    fn day(&self) -> usize {
        2
    }

    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self) {
        let file =
            File::open("./input/day-02.txt").expect("Pour avancer, il faut le fichier day-02.txt");
        let buffer = BufReader::new(file);

        self.games = buffer
            .lines()
            .map(|l| l.unwrap().split_once(':').unwrap().1.to_string())
            .map(|game| {
                game.split(';')
                    .map(|raw_round| {
                        let mut round = Round::default();
                        let cubes_iterator = raw_round.split(',').filter_map(|s| {
                            s.trim().split_once(' ').and_then(|(n, color)| {
                                n.parse::<u32>().map(|number| (number, color)).ok()
                            })
                        });

                        for (number, color) in cubes_iterator {
                            match color {
                                "green" => round.green = number,
                                "blue" => round.blue = number,
                                "red" => round.red = number,
                                _ => unreachable!(), }
                        }

                        round
                    })
                    .collect::<Vec<Round>>()
            })
            .collect::<Vec<Vec<Round>>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut valid_games_sum = 0;
        'outer: for (i, game) in self.games.iter().enumerate() {
            for round in game {
                if round.blue > self.reference_bag.blue
                    || round.green > self.reference_bag.green
                    || round.red > self.reference_bag.red
                {
                    continue 'outer;
                }
            }
            valid_games_sum += i + 1;
        }

        aoclib::output(valid_games_sum)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("")
    }
}

#[test]
fn day_02_file_reading() {
    let mut day = AocDay02::default();
    day.parse();

    assert_eq!(day.games.get(0..3).unwrap(), vec![
            vec![
                Round { green: 1, blue: 7, red: 9 },
                Round { green: 8, blue: 0, red: 0 },
                Round { green: 10, blue: 5, red: 3 },
                Round { green: 1, blue: 11, red: 5  }
            ],
            vec![
                Round { green: 7, blue: 3, red: 0 },
                Round { green: 4, blue: 20, red: 0 },
                Round { green: 2, blue: 13, red: 6  }
            ],
            vec![
                Round { green: 1, blue: 11, red: 3 },
                Round { green: 3, blue: 9, red: 15 },
                Round { green: 4, blue: 11, red: 4 },
                Round { green: 2, blue: 14, red: 1 },
                Round { green: 4, blue: 18, red: 10  }
            ]
        ]
    );
}

#[test]
fn day_02_part1() {
    /*
    Input:
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    */

    let mut day = AocDay02 {
        reference_bag: Round {
            green: 13,
            blue: 14,
            red: 12,
        },
        games: vec![
            vec![
                Round { blue: 3, red: 4, green: 0, },
                Round { blue: 6, red: 1, green: 2, },
                Round { blue: 0, red: 0, green: 2, },
            ],
            vec![
                Round { blue: 1, red: 0, green: 2, },
                Round { blue: 4, red: 1, green: 3, },
                Round { blue: 1, red: 0, green: 1, },
            ],
            vec![
                Round { blue: 6, red: 20, green: 8, },
                Round { blue: 5, red: 4, green: 13, },
                Round { blue: 0, red: 1, green: 5, },
            ],
            vec![
                Round { blue: 6, red: 3, green: 1, },
                Round { blue: 0, red: 6, green: 3, },
                Round { blue: 15, red: 14, green: 3, },
            ],
            vec![
                Round { blue: 1, red: 6, green: 3, },
                Round { blue: 2, red: 1, green: 2, },
            ],
        ],
    };

    assert_eq!(day.part1(), vec!["8".to_string()]);
}

#[test]
fn day_02_part2() {
}
