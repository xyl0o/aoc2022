use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryInto;
use std::io::{Error, ErrorKind};

use std::str::FromStr;

use trees::Tree;

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!(
        "Part one: {:?}",
        part_one_solution
    );

    let part_two_solution = part_two(input);
    println!(
        "Part two: {:?}",
        part_two_solution
    );
}

pub fn part_one(input: &str) -> u32 {
    todo!();
}

pub fn part_two(input: &str) -> u32 {
    todo!();
}

fn parse_args(input: &str) -> Vec<&str>{
    input
        .split(' ')
        .filter(|arg| { arg != &"" })
        .collect()
}

fn parse_cmd(input: &str) -> Result<Cmd, &'static str> {
    lazy_static! {
        static ref RE_CMD: Regex =
            Regex::new(r"^\$ ([[:word:]]+)((?:\s+*\S+)*)\s*").unwrap();
    }

    let mut lines = input.lines();

    let caps = RE_CMD
        .captures(lines.next().ok_or("Input does not contain a line")?)
        .ok_or("No cmd found")?;

    // we know that the first group is not None bc. whole regex matches
    let cmd = match caps.get(1).unwrap().as_str() {
        "ls" => "ls",
        "cd" => "cd",
        _ => return Err("Unknown cmd"),
    };

    let args = match caps.get(2) {
        Some(args_str) => parse_args(args_str.as_str()),
        None => Vec::new(),
    };

    if cmd == "ls" {
        if args.len() != 0 {
            return Err("ls doesn't take any args")
        }
        return Ok(Cmd::Ls {
            output: lines.collect()
        });
    }

    if cmd == "cd" {
        if args.len() != 1 {
            return Err("cd takes exactly one argument")
        }

        return Ok(Cmd::Cd {
            args: args.try_into().unwrap()
        });
    }

    Err("Unable to parse cmd")
}


#[derive(Debug)]
enum FsObj {
    Directory{
        name: String,
    },
    File{
        name: String,
        size: u32,
    },
}

impl FromStr for FsObj {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        lazy_static! {
            static ref RE_LINE: Regex =
                Regex::new(r"([0-9]+|dir) ([[:print:]]+)").unwrap();
        }

        let caps = RE_LINE.captures(s).ok_or(
            Self::Err::new(ErrorKind::InvalidData, "Coulnd't parse output"))?;

        let fname = caps.get(2).unwrap().as_str().to_string();

        let fs_obj = match caps.get(1).unwrap().as_str() {
            "dir" => FsObj::Directory{ name: fname },
            fsize => FsObj::File{
                name: fname,
                size: fsize.parse().map_err(|_|
                    Self::Err::new(ErrorKind::InvalidData, "Coulnd't parse output"))?,
            },
        };
        Ok(fs_obj)
    }
}

fn reconstruct_folder_tree(cmds: &Vec<Cmd>) -> Result<Tree<FsObj>, Error>{
    let mut tree = Tree::new(FsObj::Directory{ name: "/".to_string() });

    let mut cwd = tree.root_mut();

    let mut cmds_iter = cmds.iter();

    while let Some(cmd) = cmds_iter.next() {
        match cmd {
            Cmd::Ls { output } => {
                for l in output {
                    cwd.push_back(Tree::new(l.parse()?));
                }
            },
            Cmd::Cd { args } if args[0] == ".." => cwd = cwd.parent().unwrap_or(tree.root_mut()),
            Cmd::Cd { args } if args[0] == "/" => cwd = tree.root_mut(),
            Cmd::Cd { args } => cwd = cwd
                .iter()
                .filter(|node| match node.data() {
                    FsObj::Directory { name } => name == args[0],
                    FsObj::File { name, size } => false,
                })
                .next()
                .expect("Illegal file operation"),
        }
    }

    Ok(tree)
}

#[derive(Debug, PartialEq)]
enum Cmd<'a> {
    Cd {args: [&'a str; 1]},
    Ls {output: Vec<&'a str>},
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_cmd_ls() {
        let input = indoc! {"
            $ ls
            123 a
            321 b
            dir c
        "};
        assert_eq!(parse_cmd(input), Ok(
            Cmd::Ls {
                output: vec![
                    "123 a",
                    "321 b",
                    "dir c",
                ],
            }
        ));
    }

    #[test]
    fn test_parse_cmd_cd() {
        let input = "$ cd ..";
        assert_eq!(parse_cmd(input), Ok(Cmd::Cd { args: [".."] }));
        let input = "$ cd /   ";
        assert_eq!(parse_cmd(input), Ok(Cmd::Cd { args: ["/"] }));
    }

    #[test]
    fn test_parse_cmd_invalid() {
        let input = "$ foo /";
        assert_eq!(parse_cmd(input), Err("Unknown cmd"));
    }

    #[test]
    fn test_parse_cmd_ls_with_args() {
        let input = "$ ls /";
        assert_eq!(parse_cmd(input), Err("ls doesn't take any args"));
        let input = "$ ls   ";
        assert_eq!(parse_cmd(input), Ok(Cmd::Ls {output: vec![]}));
    }

    #[test]
    fn test_parse_cmd_cd_with_too_few_or_many_args() {
        let input = "$ cd";
        assert_eq!(parse_cmd(input), Err("cd takes exactly one argument"));
        let input = "$ cd      ";
        assert_eq!(parse_cmd(input), Err("cd takes exactly one argument"));
        let input = "$ cd / ..   ";
        assert_eq!(parse_cmd(input), Err("cd takes exactly one argument"));
    }
}
