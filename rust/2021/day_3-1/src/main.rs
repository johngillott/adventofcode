use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let mut file = File::open(dbg!(config.filename)).expect("Unable to open file");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to load file contents {}");

    let ab = input
        .lines()
        .map(|s| s.to_string().into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let ba = transpose(ab);

    let num_bit = ba.first().expect("expect ba have one element").len() / 2;

    let r_gamma = ba
        .iter()
        .map(|x| {
            let r = match x.iter().filter(|x| **x == 48u8).count() > num_bit {
                true => 48u8,
                false => 49u8,
            };

            r
        })
        .collect::<Vec<u8>>();

    let gamma_rate = u32::from_str_radix(str::from_utf8(&r_gamma).unwrap(), 2)
        .expect("expect r_game as decimal number");

    let r_epsilon = r_gamma
        .iter()
        .map(|x| match *x == 48u8 {
            true => 49u8,
            false => 48u8,
        })
        .collect::<Vec<u8>>();

    let epsilon_rate = u32::from_str_radix(str::from_utf8(&r_epsilon).unwrap(), 2)
        .expect("expect r_epsilon as decimal number");

    println!(
        "gamma_rate={} epsilon_rate={}, power consumption={}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}

fn transpose(ab: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let a_len = ab.len();
    let b_len = ab
        .first()
        .expect("list should contain at least 1 element")
        .len();

    let mut ba: Vec<_> = iter::repeat_with(|| vec![0; a_len]).take(b_len).collect();

    // TODO: Address clippy warning
    for i in 0..b_len {
        for j in 0..a_len {
            ba[i][j] = ab[j][i];
        }
    }

    ba
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();

        Config { filename }
    }
}
