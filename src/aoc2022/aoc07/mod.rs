// use std::collections::{hash_map::Keys, HashMap};

// static INPUT: &'static str = include_str!("./input");

// #[derive(Clone, Debug)]
// struct Tree {
//     dirs: HashMap<String, Box<Tree>>,
//     files: HashMap<String, usize>,
// }

// impl Tree {
//     fn new() -> Self {
//         Self {
//             dirs: HashMap::new(),
//             files: HashMap::new(),
//         }
//     }

//     fn iter(&self) -> TreeIterator {
//         TreeIterator {
//             tree: self.clone(),
//             path: self.dirs.clone().keys().cloned(),
//             has_returned_self: false,
//         }
//     }

//     fn size(&self) -> usize {
//         self.files.iter().map(|(_, v)| v).sum::<usize>()
//             + self.dirs.iter().map(|(_, t)| t.size()).sum::<usize>()
//     }
// }

// // impl IntoIterator for Tree {
// //     type Item = Tree;

// //     type IntoIter = TreeIterator<'_>

// //     fn into_iter(self) -> Self::IntoIter {
// //         self.iter()
// //     }
// // }

// struct TreeIterator<'a> {
//     tree: Tree,
//     path: std::iter::Cloned<Keys<'a, String, Box<Tree>>>,
//     has_returned_self: bool,
// }

// impl Iterator for TreeIterator<'_> {
//     type Item = Tree;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(path) = self.path.next() {
//             return self.tree.dirs.get(&path).map(|t| *t.to_owned());
//         } else if !self.has_returned_self {
//             self.has_returned_self = true;
//             return Some(self.tree.clone());
//         }

//         None
//     }
// }

// impl Default for Tree {
//     fn default() -> Self {
//         Tree::new()
//     }
// }

// fn parse() -> Tree {
//     let mut tree = Tree::default();
//     let mut cwd: Vec<String> = vec![];

//     for line in INPUT.lines() {
//         let mut split = line.split(" ");

//         let (fst, snd, thd) = (split.next().unwrap(), split.next(), split.next());

//         match fst {
//             "$" => match snd {
//                 Some("ls") => continue,
//                 Some("cd") => match thd {
//                     Some("/") => cwd = vec![],
//                     Some("..") => {
//                         cwd.pop();
//                         ()
//                     }
//                     Some(dir) => cwd.push(dir.to_string()),
//                     _ => panic!("Wrong use of cd"),
//                 },
//                 _ => panic!("Empty command"),
//             },
//             "dir" => match snd {
//                 Some(dir) => insert_dir_at(dir.to_string(), &mut tree, cwd.clone()),
//                 _ => panic!("Empty dir"),
//             },
//             size => match snd {
//                 Some(file) => insert_file_at(
//                     file.to_string(),
//                     size.parse().unwrap(),
//                     &mut tree,
//                     cwd.clone(),
//                 ),
//                 _ => panic!("Invalid file listing {line}"),
//             },
//         }
//     }

//     tree
// }

// fn insert_dir_at(name: String, tree: &mut Tree, path: Vec<String>) {
//     let mut tree: &mut Tree = tree;
//     // println!("insert dir {name} at {:?} in tree {:#?}", path, tree);

//     for dir in path {
//         tree = tree.dirs.get_mut(&dir).unwrap();
//     }

//     tree.dirs.insert(name, Box::new(Tree::new()));
// }

// fn insert_file_at(name: String, size: usize, tree: &mut Tree, path: Vec<String>) {
//     let mut tree: &mut Tree = tree;
//     // println!("insert file {name} at {:?} in tree {:#?}", path, tree);

//     for dir in path {
//         tree = tree.dirs.get_mut(&dir).unwrap();
//     }

//     tree.files.insert(name, size);
// }

// pub fn one() -> Option<String> {
//     let root = parse();
//     println!("final tree {:#?}", root);

//     let result = &root
//         .iter()
//         .map(|t| t.size())
//         .filter(|s| s > &100000)
//         .sum::<usize>();

//     Some(result.to_string())
// }

// pub fn two() -> Option<String> {
//     todo!()
// }
