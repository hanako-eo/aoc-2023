use std::time::{Instant, Duration};
use std::fmt::Display;

pub fn hello() {
    println!("Hello in AOClib !");
}

pub enum Selector {
    All,
    One(usize),
    Last,
}

pub trait Runner {
    fn parse(&mut self);
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
}

pub fn run_solution<T: Runner + ?Sized>(solution: &mut T, day: usize) {
    println!("---- AoC 2023, Day {} ----", day);

    let start = Instant::now();
    solution.parse();
    let parse_time = start.elapsed().as_millis();
    println!("{:3}.{:03} Parsing", parse_time / 1000, parse_time % 1000);

    let start = Instant::now();
    let p1 = solution.part1();
    let p1_time = start.elapsed();
    print_solution(1, &p1, p1_time);

    let start = Instant::now();
    let p2 = solution.part2();
    let p2_time = start.elapsed();
    print_solution(2, &p2, p2_time);
}

fn print_solution<T: Display>(which: usize, output: &Vec<T>, duration: Duration) {
    let ms = duration.as_millis();
    let sec_part = ms / 1000;
    let ms_part = ms % 1000;

    let mut i = output.iter();
    println!(
        "{sec_part:3}.{ms_part:03} Part {which}: {}",
        i.next().unwrap()
    );
    for line in i {
        println!("{:16}{line}", "");
    }
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
}
