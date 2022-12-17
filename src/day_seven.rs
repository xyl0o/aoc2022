use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::io::{Error, ErrorKind};

use std::str::FromStr;

use rctree::Node;

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!("Part one: {:?}", part_one_solution);

    let part_two_solution = part_two(input);
    println!("Part two: {:?}", part_two_solution);
}

pub fn part_one(input: &str) -> u32 {
    lazy_static! {
        static ref RE_CMD_RAW: Regex = Regex::new(r"(?m)^\$(?:[^\$])+").unwrap();
    }

    let cmds: Result<Vec<_>, _> = RE_CMD_RAW
        .find_iter(input)
        .map(|m| parse_cmd(m.as_str()))
        .collect();

    let tree = FsTree::construct(&cmds.unwrap()).unwrap();
    let sum: u32 = tree
        .tree
        .descendants()
        .filter_map(|n| match &*n.borrow() {
            FsObj::Directory { name: _, total } => Some(total.to_owned()),
            FsObj::File { name: _, size: _ } => None,
        })
        .filter(|size| size <= &100000)
        .sum();
    sum
}

pub fn part_two(input: &str) -> u32 {
    todo!();
}

fn parse_args(input: &str) -> Vec<&str> {
    input.split(' ').filter(|arg| arg != &"").collect()
}

#[derive(Debug, PartialEq)]
enum Cmd<'a> {
    Cd { path: &'a str },
    Ls { output: Vec<&'a str> },
}

fn parse_cmd(input: &str) -> Result<Cmd, &'static str> {
    lazy_static! {
        static ref RE_CMD: Regex = Regex::new(r"^\$ ([[:word:]]+)((?:\s+*\S+)*)\s*").unwrap();
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
            return Err("ls doesn't take any args");
        }
        return Ok(Cmd::Ls {
            output: lines.collect(),
        });
    }

    if cmd == "cd" {
        if args.len() != 1 {
            return Err("cd takes exactly one argument");
        }

        return Ok(Cmd::Cd { path: args[0] });
    }

    Err("Unable to parse cmd")
}

#[derive(Debug, PartialEq)]
enum FsObj {
    Directory { name: String, total: u32 },
    File { name: String, size: u32 },
}

impl FromStr for FsObj {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        lazy_static! {
            static ref RE_LINE: Regex = Regex::new(r"([0-9]+|dir) ([[:print:]]+)").unwrap();
        }

        let caps = RE_LINE.captures(s).ok_or(Self::Err::new(
            ErrorKind::InvalidData,
            "Couldn't parse output",
        ))?;

        let fname = caps.get(2).unwrap().as_str().to_string();

        let fs_obj = match caps.get(1).unwrap().as_str() {
            "dir" => FsObj::Directory {
                name: fname,
                total: 0,
            },
            fsize => FsObj::File {
                name: fname,
                size: fsize
                    .parse()
                    .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Couldn't parse output"))?,
            },
        };
        Ok(fs_obj)
    }
}

impl fmt::Display for FsObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsObj::Directory { name, total } => write!(f, "dir {} ({})", name, total),
            FsObj::File { name, size } => write!(f, "{} {}", size, name),
        }
    }
}

type FsNode = Node<FsObj>;

#[derive(Debug)]
struct FsTree {
    pub(crate) tree: FsNode,
}

impl FsTree {
    fn construct(cmds: &Vec<Cmd>) -> Result<FsTree, Error> {
        let root = Node::new(FsObj::Directory {
            name: "/".to_string(),
            total: 0,
        });

        let mut cwd = root.clone();
        let mut cmds_iter = cmds.iter();

        while let Some(cmd) = cmds_iter.next() {
            cwd = match cmd {
                Cmd::Ls { output } => {
                    for l in output {
                        Self::append_obj(&cwd, l.parse()?);
                    }
                    cwd
                }
                Cmd::Cd { path } if path == &".." => cwd.ancestors().take(2).last().unwrap(),
                Cmd::Cd { path } if path == &"/" => cwd.ancestors().last().unwrap(),
                Cmd::Cd { path } => cwd
                    .children()
                    .filter(|node| match &*node.borrow() {
                        FsObj::Directory { name, total: _ } => &name == path,
                        FsObj::File { name: _, size: _ } => false,
                    })
                    .next()
                    .expect("Illegal file operation"),
            }
        }

        Ok(FsTree { tree: root })
    }

    fn propagate_size(node: &FsNode, size: u32) {
        let mut node_data = node.borrow_mut();
        let node_size = match &mut *node_data {
            FsObj::Directory {
                name: _,
                ref mut total,
            } => total,
            FsObj::File { name: _, size: _ } => panic!("can't insert into file"),
        };
        *node_size += size;
        drop(node_data);

        if let Some(ref parent) = node.parent() {
            Self::propagate_size(parent, size);
        };
    }

    fn append_obj(node: &FsNode, obj: FsObj) {
        let data = node.borrow();
        if let FsObj::File { name: _, size: _ } = &*data {
            panic!("can't append to file");
        }
        drop(data);

        let size = match obj {
            FsObj::Directory { name: _, total } => total,
            FsObj::File { name: _, size } => size,
        };

        Self::propagate_size(node, size);

        node.append(FsNode::new(obj));
    }
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
        assert_eq!(
            parse_cmd(input),
            Ok(Cmd::Ls {
                output: vec!["123 a", "321 b", "dir c",],
            })
        );
    }

    #[test]
    fn test_parse_cmd_cd() {
        let input = "$ cd ..";
        assert_eq!(parse_cmd(input), Ok(Cmd::Cd { path: ".." }));
        let input = "$ cd /   ";
        assert_eq!(parse_cmd(input), Ok(Cmd::Cd { path: "/" }));
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
        assert_eq!(parse_cmd(input), Ok(Cmd::Ls { output: vec![] }));
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

    #[test]
    fn test_fs_tree_from_cmd_vec() {
        // / (dir, total=18445)
        // - a (file, size=123)
        // - b (dir, total=18001)
        //   - foo.txt (file, size=9000)
        //   - bar.txt (file, size=9001)
        // - c (file, size=321)
        // - d (dir, total=0)
        let input = vec![
            Cmd::Ls {
                output: vec!["123 a", "dir b", "321 c", "dir d"],
            },
            Cmd::Cd { path: &"b" },
            Cmd::Ls {
                output: vec!["9000 foo.txt", "9001 bar.txt"],
            },
        ];
        let tree = FsTree::construct(&input);
        assert!(tree.is_ok(), "Could not build tree");

        let mut tree_iter = tree.unwrap().tree.descendants();

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain /");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::Directory {
                name: "/".to_owned(),
                total: 18445,
            }
        );

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain a");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::File {
                name: "a".to_owned(),
                size: 123,
            }
        );

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain c");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::Directory {
                name: "b".to_owned(),
                total: 18001,
            }
        );

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain foo.txt");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::File {
                name: "foo.txt".to_owned(),
                size: 9000
            }
        );

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain bar.txt");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::File {
                name: "bar.txt".to_owned(),
                size: 9001
            }
        );

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain c");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::File {
                name: "c".to_owned(),
                size: 321
            }
        );

        let node: Option<FsNode> = tree_iter.next();
        assert!(node.is_some(), "Tree doesn't contain d");

        assert_eq!(
            &*(node.unwrap()).borrow(),
            &FsObj::Directory {
                name: "d".to_owned(),
                total: 0,
            }
        );
    }
}
