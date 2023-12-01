use aoclib::Runner;

#[derive(Default)]
pub struct AocDay00 {
    lines: Vec<String>,
    matrixs: Vec<Vec<String>>
}

impl AocDay00 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for AocDay00 {
    fn day(&self) -> usize { 0 }

    // fonction pour parser le contenu d'un fichier pour un jour précis
    fn parse(&mut self) {
        self.lines = vec![
            "C.....s".to_string(),
            "C...s".to_string(),
            "s..C".to_string(),
            ".....".to_string(),
            "C".to_string(),
            "Cs".to_string(),
            "s".to_string(),
            "...s....C..".to_string(),
        ];
        self.matrixs = vec![
            vec!["..C...s..".to_string()],
            vec!["..C......".to_string(),
                 "....s....".to_string()],
            vec!["........C".to_string(),
                 ".........".to_string(),
                 ".........".to_string()],
            vec![".C.......".to_string(),
                 ".........".to_string(),
                 "......s..".to_string()],
            vec!["........C".to_string(),
                 "s........".to_string(),
                 ".........".to_string()],
            vec![".........".to_string(),
                 "...s.....".to_string(),
                 ".........".to_string()],
            vec!["........C".to_string(),
                 "........s".to_string(),
                 ".........".to_string()],
            vec![".........".to_string(),
                 ".........".to_string(),
                 "...C.s...".to_string()],
        ];
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.lines
            .iter()
            .map(|line| {
                let mut mouse_pos: Option<u64> = None;
                let mut cat_pos: Option<u64> = None;

                for (i, c) in line.char_indices() {
                    if c == 'C' {
                        cat_pos = Some(i as u64)
                    } else if c == 's' {
                        mouse_pos = Some(i as u64)
                    }
                }

                match (mouse_pos, cat_pos) {
                    (Some(mouse_pos), Some(cat_pos)) => cat_pos.abs_diff(mouse_pos),
                    _ => 0
                }
            })
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(""))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.matrixs
            .iter()
            .map(|matrix| {
                let mut mouse_pos: Option<(u64, u64)> = None;
                let mut cat_pos: Option<(u64, u64)> = None;

                for (i, line) in matrix.iter().enumerate() {
                    for (j, c) in line.char_indices() {
                        if c == 'C' {
                            cat_pos = Some((i as u64, j as u64))
                        } else if c == 's' {
                            mouse_pos = Some((i as u64, j as u64))
                        }
                    }
                }

                match (mouse_pos, cat_pos) {
                    (Some(mouse_pos), Some(cat_pos)) => cat_pos.0.abs_diff(mouse_pos.0) + cat_pos.1.abs_diff(mouse_pos.1),
                    _ => 0
                }
            })
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(""))
    }
}

#[test]
fn day_00_part1() {
    let mut day = AocDay00 {
        matrixs: Default::default(),
        lines: vec![
            "C.....s".to_string(),
            "C...s".to_string(),
            "s..C".to_string(),
            ".....".to_string(),
            "C".to_string(),
            "Cs".to_string(),
            "s".to_string(),
        ],
    };

    assert_eq!(day.part1(), vec!["6430010".to_string()])
}

#[test]
fn day_00_part2() {
    let mut day = AocDay00 {
        lines: Default::default(),
        matrixs: vec![
            vec!["..C...s..".to_string()],
            vec!["..C......".to_string(),
                 "....s....".to_string()],
            vec!["........C".to_string(),
                 ".........".to_string(),
                 ".........".to_string()],
            vec![".C.......".to_string(),
                 ".........".to_string(),
                 "......s..".to_string()],
            vec!["........C".to_string(),
                 "s........".to_string(),
                 ".........".to_string()],
            vec![".........".to_string(),
                 "...s.....".to_string(),
                 ".........".to_string()],
            vec!["........C".to_string(),
                 "........s".to_string(),
                 ".........".to_string()],
            vec![".........".to_string(),
                 ".........".to_string(),
                 "...C.s...".to_string()],
        ],
    };

    assert_eq!(day.part2(), vec!["43079012".to_string()])
}
