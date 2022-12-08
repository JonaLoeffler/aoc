use std::{collections::HashMap, pin::Pin, str::Lines};

static INPUT: &'static str = include_str!("./input");

#[derive(Clone, Debug)]
struct Tree {
    dirs: HashMap<String, Box<Tree>>,
    files: HashMap<String, usize>,
}

impl Tree {
    fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Tree::new()
    }
}

fn parse_recursive(iter: &mut Lines, current: Option<&mut Tree>) -> Tree {
    let mut default = Tree::default();
    let root = current.unwrap_or(&mut default);

    while let Some(line) = iter.next() {
        println!("Next line: {:#?}", line);
        let mut split = line.split(" ");

        let (fst, snd, thd) = (split.next().unwrap(), split.next(), split.next());
        match fst {
            "$" => match snd {
                Some("ls") => return parse_recursive(iter, Some(root)),
                Some("cd") => {
                    return if thd == Some("/") {
                        parse_recursive(iter, Some(root))
                    } else if thd == Some("..") {
                        continue;
                    } else {
                        parse_recursive(iter, Some(root.dirs.get_mut(thd.unwrap()).unwrap()))
                    };
                }
                _ => panic!("Empty command"),
            },
            "dir" => {
                println!("insert dir {:#?}", snd.unwrap());
                root.dirs
                    .insert(snd.unwrap().to_string(), Box::new(Tree::new()));
                println!("{:#?}", root);

                return parse_recursive(iter, Some(root));
            }
            _ => {
                println!("insert file {:#?}", snd.unwrap());

                root.files
                    .insert(snd.unwrap().to_string(), fst.parse::<usize>().unwrap());
                println!("{:#?}", root);

                return parse_recursive(iter, Some(root));
            }
        }
    }

    return root.clone();
}

fn parse() -> Tree {
    parse_recursive(&mut INPUT.lines(), None)
}

pub fn one() -> Option<String> {
    let root = parse();
    println!("{:#?}", root);

    None
}

pub fn two() -> Option<String> {
    None
}
