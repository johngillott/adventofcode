use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::ops::{Index, IndexMut};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let mut file = File::open(dbg!(config.filename)).expect("Unable to open file");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to load file contents {}");

    let (mut max_x, mut max_y) = (0, 0);

    // parse input
    let coords = input
        .lines()
        .map(|s| {
            let s = s
                .replace(" -> ", ",")
                .split(',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            max_x = max_x.max(s[0]).max(s[2]);

            max_y = max_x.max(s[1]).max(s[3]);

            ((s[0], s[1]), (s[2], s[3]))
        })
        .collect::<Vec<((isize, isize), (isize, isize))>>();

    // initialize grid
    let mut grid: Grid = iter::repeat_with(|| vec![0; max_x as usize + 1])
        .take(max_y as usize + 1)
        .collect::<Vec<Vec<isize>>>()
        .into();

    coords.into_iter().for_each(|c| {
        let ((x0, y0), (x1, y1)) = c;
        if x0 == x1 {
            let (min_y, max_y) = if y0 > y1 { (y1, y0) } else { (y0, y1) };

            for y in min_y..=max_y {
                grid[x0.try_into().unwrap()][y as usize] += 1;
            }
        } else if y0 == y1 {
            let (min_x, max_x) = if x0 > x1 { (x1, x0) } else { (x0, x1) };

            for x in min_x..=max_x {
                grid[x as usize][y0 as usize] += 1;
            }
        }
    });

    let num_of_intersections = grid.into_iter().flatten().filter(|n| *n >= 2).count();
    println!(
        "Number of intersections (2 or more) - a total of {} points",
        num_of_intersections,
    );
}

struct Grid(Vec<Vec<isize>>);

impl Grid {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Vec<isize>>> for Grid {
    fn from(vec: Vec<Vec<isize>>) -> Self {
        Grid(vec)
    }
}

impl Index<usize> for Grid {
    type Output = Vec<isize>;

    fn index(&self, index: usize) -> &Vec<isize> {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IntoIterator for Grid {
    type Item = Vec<isize>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len() {
            writeln!(f, "{:?}", self[i])?;
        }

        Ok(())
    }
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
