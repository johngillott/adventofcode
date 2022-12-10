use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Line> {
    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
        .collect::<Vec<_>>();

    lines
}

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> u64 {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child).unwrap());
    }
    total
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Line]) -> u64 {
    let mut tree = Tree::<FsEntry>::new();
    let root = tree
        .insert(
            Node::new(FsEntry {
                path: "/".into(),
                size: 0,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();
    let mut curr = root;

    for line in input.iter() {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry {
                        size: *size,
                        path: path.clone(),
                    });
                    tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                }
            },
        }
    }

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n))
        .filter(|&s| s <= 100_000)
        .sum::<u64>();

    sum
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Line]) -> u64 {
    let mut tree = Tree::<FsEntry>::new();
    let root = tree
        .insert(
            Node::new(FsEntry {
                path: "/".into(),
                size: 0,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();
    let mut curr = root;

    for line in input.iter() {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry {
                        size: *size,
                        path: path.clone(),
                    });
                    tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                }
            },
        }
    }

    let total_space = 70000000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap()).unwrap());
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n))
        .filter(|&s| s >= minimum_space_to_free)
        .min()
        .unwrap();

    size_to_remove
}

// AOC 2022
// Day 7 - Part 1 : 1648397
//         generator: 253.415µs,
//         runner: 258.824µs

// [/Users/john.gillott/dev/github.com/johngillott/adventofcode/rust/advent-of-code-2022/src/day7.rs:225] used_space = 41735494
// Day 7 - Part 2 : 1815525
//         generator: 283.969µs,
//         runner: 207.337µs

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 95437);
    }

    #[test]
    fn sample2() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 24933642);
    }
}
