use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::str;

const ZERO: u8 = 48u8;
const ONE: u8 = 49u8;

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

    let ba = transpose(&ab);

    let num_bits_ba = ba.first().expect("expect ba have one element").len();
    let num_bits_div_2_ba = num_bits_ba / 2;

    let r_gamma = ba
        .iter()
        .map(
            |x| match x.iter().filter(|x| **x == ZERO).count() > num_bits_div_2_ba {
                true => ZERO,
                false => ONE,
            },
        )
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

    let mut oxygen_generator_sieve = ab.clone();
    for i in 0..num_bits_ba {
        let sieve_len = oxygen_generator_sieve.len();
        if oxygen_generator_sieve.len() == 1 {
            break;
        }
        // count phase
        let (mut num_0, mut num_1) = (0, 0);
        for j in 0..sieve_len {
            if oxygen_generator_sieve[j][i] == ZERO {
                num_0 += 1;
            } else {
                num_1 += 1;
            }
        }

        let filter = match num_1 >= num_0 {
            true => ZERO,
            false => ONE,
        };

        // drain phase
        let mut idx = 0;
        while idx < oxygen_generator_sieve.len() {
            if oxygen_generator_sieve[idx][i] == filter {
                oxygen_generator_sieve.remove(idx);
            } else {
                idx += 1;
            }
        }
    }

    let r_oxygen_generator_rating = oxygen_generator_sieve
        .first()
        .expect("expected at least 1 element in oxygen_generator_sieve");

    let oxygen_generator_rating =
        u32::from_str_radix(str::from_utf8(&r_oxygen_generator_rating).unwrap(), 2)
            .expect("expect r_oxygen_generator_rating as decimal number");

    let mut co2_scrubber_sieve = ab.clone();
    for i in 0..num_bits_ba {
        let sieve_len = co2_scrubber_sieve.len();
        if co2_scrubber_sieve.len() == 1 {
            break;
        }
        // count phase
        let (mut num_0, mut num_1) = (0, 0);
        for j in 0..sieve_len {
            if co2_scrubber_sieve[j][i] == ZERO {
                num_0 += 1;
            } else {
                num_1 += 1;
            }
        }

        let filter = match num_0 > num_1 {
            true => ZERO,
            false => ONE,
        };

        // drain phase
        let mut idx = 0;
        while idx < co2_scrubber_sieve.len() {
            if co2_scrubber_sieve[idx][i] == filter {
                co2_scrubber_sieve.remove(idx);
            } else {
                idx += 1;
            }
        }
    }

    let r_co2_scrubber_rating = co2_scrubber_sieve
        .first()
        .expect("expected at least 1 element in co2_scrubber_sieve");

    let co2_scrubber_rating =
        u32::from_str_radix(str::from_utf8(&r_co2_scrubber_rating).unwrap(), 2)
            .expect("expect r_co2_scrubber_rating as decimal number");

    // Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.

    println!(
        "gamma_rate={} epsilon_rate={}, power consumption={}\noxygen generator rating={} CO2 scrubber rating={}, life support rating={}\n",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate,
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating,
    );
}

fn transpose(ab: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let a_len = ab.len();
    let b_len = ab
        .first()
        .expect("list should contain at least 1 element")
        .len();

    let mut ba: Vec<_> = iter::repeat_with(|| vec![0; a_len]).take(b_len).collect();

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
