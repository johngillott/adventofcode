use std::env;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let mut file = File::open(dbg!(config.filename)).expect("Unable to open file");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to load file contents {}");

    let mut school = input
        .lines()
        .map(|s| {
            s.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .flatten()
        .map(Lanternfish::new)
        .collect::<Vec<Lanternfish>>();

    for _ in 0..config.num_of_days {
        let mut new_fish = school
            .iter_mut()
            .map(|l| l.cycle())
            .flatten()
            .collect::<Vec<Lanternfish>>();

        school.append(&mut new_fish);
    }

    println!(
        "The number of fish in the school after {} day(s) is {}.",
        config.num_of_days,
        school.len()
    );
}

#[derive(Debug)]
struct Lanternfish {
    internal_timer: i64,
}

impl Lanternfish {
    pub fn new(internal_timer: i64) -> Self {
        Self { internal_timer }
    }

    pub fn cycle(&mut self) -> Option<Self> {
        self.internal_timer -= 1;

        match self.internal_timer {
            -1 => {
                self.internal_timer = 6;

                Some(Lanternfish::new(8))
            }
            _ => None,
        }
    }
}

impl fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal_timer)
    }
}

struct Config {
    filename: String,
    num_of_days: i64,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let num_of_days = args[2].parse::<i64>().unwrap();

        Config {
            filename,
            num_of_days,
        }
    }
}
