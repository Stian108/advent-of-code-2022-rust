use parse_display::FromStr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

#[derive(Debug, FromStr)]
pub enum Inp {
    #[display("$ ls")]
    Ls,
    #[display("$ cd ..")]
    Back,
    #[display("$ cd {0}")]
    Cd(String),
    #[display("dir {0}")]
    Dir(String),
    #[display("{0} {1}")]
    File(usize, String),
}

use Inp::*;

type Input = Rc<RefCell<TreeNode>>;

pub struct TreeNode {
    value: Option<usize>,
    children: HashMap<String, Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

pub fn parse_input(input: &str) -> Input {
    let inp = parse_lines::<Inp, Vec<_>>(input);
    let root = Rc::new(RefCell::new(TreeNode {
        value: None,
        children: HashMap::new(),
        parent: None,
    }));
    let mut tree = Rc::clone(&root);
    for inp in inp.iter().skip(1) {
        match inp {
            Ls => {}
            Back => {
                let parent = Rc::clone(tree.borrow().parent.as_ref().unwrap());
                tree = parent;
            }
            Cd(dir) => {
                let child = Rc::clone(tree.borrow().children.get(dir).unwrap());
                tree = child;
            }
            Dir(label) => {
                let new = Rc::new(RefCell::new(TreeNode {
                    value: None,
                    children: HashMap::new(),
                    parent: Some(Rc::clone(&tree)),
                }));
                tree.borrow_mut().children.insert(label.clone(), new);
            }
            File(size, label) => {
                let new = Rc::new(RefCell::new(TreeNode {
                    value: Some(*size),
                    children: HashMap::new(),
                    parent: Some(Rc::clone(&tree)),
                }));
                tree.borrow_mut().children.insert(label.clone(), new);
            }
        }
    }
    root
}

fn sum(tree: &Input) -> usize {
    let tree = Rc::clone(&tree);
    let tree = tree.borrow();
    if let Some(size) = tree.value {
        size
    } else {
        tree.children.iter().map(|(_, v)| sum(v)).sum()
    }
}

fn get_dirs(tree: &Input) -> Vec<Input> {
    let mut dirs = vec![];
    fn traverse(tree: &Input, dirs: &mut Vec<Input>) {
        if let Some(_) = tree.borrow().value {
            ()
        } else {
            dirs.push(Rc::clone(&tree));
            for (_, child) in tree.borrow().children.iter() {
                traverse(child, dirs)
            }
        }
    }
    traverse(tree, &mut dirs);
    dirs
}

pub fn part1(inp: &Input) -> usize {
    get_dirs(inp)
        .iter()
        .map(|tree| sum(tree))
        .filter(|sum| *sum <= 100000)
        .sum()
}

pub fn part2(inp: &Input) -> usize {
    let need = sum(inp) - 40000000;
    get_dirs(inp)
        .iter()
        .map(|tree| sum(tree))
        .filter(|sum| *sum >= need)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "$ cd /
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
7214296 k";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 95437)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 24933642)
    }
}
