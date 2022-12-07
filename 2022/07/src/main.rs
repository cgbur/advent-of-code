use aoc::input;
use itertools::Itertools;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
    is_dir: bool,
}

fn main() {
    let file_system = Rc::new(RefCell::new(Node {
        name: "/".to_string(),
        size: 0,
        children: vec![],
        parent: None,
        is_dir: true,
    }));
    let mut current_dir = file_system.clone();
    for line in input().lines() {
        if line.starts_with("$") {
            let command = line.split_whitespace().skip(1).collect_vec();
            match *command.first().unwrap() {
                "cd" => match command[1] {
                    ".." => {
                        let parent = current_dir.borrow().parent.as_ref().unwrap().clone();
                        current_dir = parent;
                    }
                    _ => {
                        if command[1] == "/" {
                            current_dir = file_system.clone();
                            continue;
                        }
                        let child_dir = current_dir
                            .borrow()
                            .children
                            .iter()
                            .find(|child| child.borrow().name == command[1])
                            .unwrap()
                            .clone();
                        current_dir = child_dir;
                    }
                },
                "ls" => {
                    // don't care going to skip
                }
                _ => {
                    panic!("unknown command {}", command[0]);
                }
            }
        } else {
            if line.starts_with("dir") {
                let name = line.split_whitespace().skip(1).collect_vec()[0].to_string();
                let new_node = Node {
                    name,
                    size: 0,
                    is_dir: true,
                    children: vec![],
                    parent: Some(current_dir.clone()),
                };
                current_dir
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(new_node)));
            } else {
                let mut parts = line.split_whitespace();
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let name = parts.next().unwrap().to_string();
                let new_node = Node {
                    name,
                    size,
                    is_dir: false,
                    children: vec![],
                    parent: Some(current_dir.clone()),
                };
                current_dir
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(new_node)));
                // update the size of current_dir and all of its parents
                current_dir.borrow_mut().size += size;
                let mut parent = current_dir.borrow().parent.clone();
                while let Some(parent_node) = parent {
                    parent_node.borrow_mut().size += size;
                    parent = parent_node.borrow().parent.clone();
                }
            }
        }
    }

    // Find all of the directories with a total size of at most 100000. What is
    // the sum of the total sizes of those directories?
    let mut total = 0;
    let mut stack = vec![file_system.clone()];
    while let Some(node) = stack.pop() {
        if node.borrow().is_dir {
            if node.borrow().size <= 100000 {
                // println!("{} {}", node.borrow().name, node.borrow().size);
                total += node.borrow().size;
            }
            for child in node.borrow().children.iter() {
                stack.push(child.clone());
            }
        }
    }
    println!("part one: {}", total);

    // Find the smallest directory that, if deleted, would free up enough space
    // on the filesystem to run the update. What is the total size of that
    // directory?
    const DISK_SIZE: usize = 70000000;
    const REQUIRED_FREE_SPACE: usize = 30000000;
    let used_space = file_system.borrow().size;
    let unused_space = DISK_SIZE - used_space;
    let amount_to_free = REQUIRED_FREE_SPACE - unused_space;

    let mut smallest_dir: Option<Rc<RefCell<Node>>> = None;
    let mut stack = vec![file_system];
    while let Some(node) = stack.pop() {
        if node.borrow().is_dir {
            if node.borrow().size >= amount_to_free {
                if let Some(smallest) = smallest_dir.as_ref() {
                    if node.borrow().size < smallest.borrow().size {
                        smallest_dir = Some(node.clone());
                    }
                } else {
                    smallest_dir = Some(node.clone());
                }
            }
            for child in node.borrow().children.iter() {
                stack.push(child.clone());
            }
        }
    }
    println!("part two: {}", smallest_dir.unwrap().borrow().size);
}
