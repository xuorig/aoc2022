use std::{cell::RefCell, rc::Rc};

fn main() {
    let input = include_str!("../inputs/7.txt");

    let root = Rc::new(RefCell::new(File { parent: None, children: vec![], size: 0, name: String::from("/") }));
    let mut current_file = root.clone();

    for line in input.trim().lines() {
        if line.starts_with("$") {
            let command = parse_command(&line);

            match command {
                Command::Cd(dir) => match dir.as_str() {
                    "/" => {},
                    ".." => {
                        let parent = current_file.borrow().parent.clone().unwrap();
                        current_file = parent;
                    },
                    _ => {
                        let selected_file = current_file.borrow().children.iter().find(|f| f.borrow().name == dir ).expect("Directory not found").clone();
                        current_file = selected_file;
                    }
                },
                Command::Ls => {
                    // NOOP
                }
            }
        } else {
            let mut split = line.split(" ");
            let (dir_or_size, name) = (split.next().unwrap(), split.next().unwrap());

            let parent_ref = current_file.clone();

            if dir_or_size == "dir" {
                current_file.borrow_mut().add_file(File { parent: Some(parent_ref), children: vec![], size: 0, name: name.to_string() });
            } else {
                let size = dir_or_size.parse::<u32>().unwrap();
                current_file.borrow_mut().add_file(File { parent: Some(parent_ref), children: vec![], size, name: name.to_string() });
            }
        }
    }

    let mut stack = Vec::new();
    let mut sizes = Vec::new();
    stack.push(root.clone());

    while stack.len() > 0 {
        let file = stack.pop().unwrap();

        if file.borrow().is_dir() {
            sizes.push(file.borrow().size());
        }

        for child in &file.borrow().children {
            stack.push(child.clone());
        }
    }

    // Paer 1
    println!("Sum: {:?}", sizes.iter().filter(|s| **s < 100000).sum::<u32>());

    // Part 2
    let root_size = root.borrow().size();
    let available_space = 70000000 - root_size;
    let smallest = sizes.iter().filter(|size| {
        *size + available_space >= 30000000
    }).min();
    println!("Smallest: {:?}", smallest);
}

fn parse_command(line: &str) -> Command {
  match &line[2..4] {
      "cd" => Command::Cd(line[5..].to_string()),
      "ls" => Command::Ls,
      _ => unreachable!()
  }
}

enum Command {
    Cd(String),
    Ls
}

type FileHandle = Rc<RefCell<File>>;

struct File {
    parent: Option<FileHandle>,
    children: Vec<FileHandle>,
    size: u32,
    name: String
}

impl File {
    fn size(&self) -> u32 {
        self.size + self.children.iter().map(|f| f.borrow().size()).sum::<u32>()
    }

    fn add_file(&mut self, f: File) {
        self.children.push(Rc::new(RefCell::new(f)));
    }

    fn is_dir(&self) -> bool {
        self.size == 0 && self.children.len() != 0
    }

}
