use std::{cell::RefCell, rc::Rc, str::FromStr};

fn main() {
    let input = include_str!("../inputs/7.txt");

    let root = Rc::new(RefCell::new(File { parent: None, children: vec![], size: 0, name: String::from("/") }));
    let mut current_file = root.clone();

    for line in input.trim().lines() {
        let output = line.parse::<FileSystemOutputLine>().unwrap();

        match output {
            FileSystemOutputLine::Command(command) => match command {
                CommandLine::Cd(dir) => match dir.as_str() {
                    "/" => {},
                    ".." => {
                        let parent = current_file.borrow().parent.clone().unwrap();
                        current_file = parent;
                    },
                    _ => {
                        let selected_file = current_file.borrow().children.iter().find(|f| f.borrow().name == dir ).expect("Directory not found").clone();
                        current_file = selected_file;
                    }
                }
                CommandLine::Ls => {}
            },
            FileSystemOutputLine::File(file) => match file {
                FileLine::Dir(dir) => {
                    current_file.borrow_mut().add_file(File { parent: Some(current_file.clone()), children: vec![], size: 0, name: dir });
                },
                FileLine::File(name, size) => {
                    current_file.borrow_mut().add_file(File { parent: Some(current_file.clone()), children: vec![], size, name });
                }
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

    // Part 1
    println!("Sum: {:?}", sizes.iter().filter(|s| **s < 100000).sum::<u32>());

    // Part 2
    let root_size = root.borrow().size();
    let available_space = 70000000 - root_size;
    let smallest = sizes.iter().filter(|size| {
        *size + available_space >= 30000000
    }).min();
    println!("Smallest: {:?}", smallest);
}

/*
 * File System
 */
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

/*
 * Parser
 */
enum FileSystemOutputLine {
    Command(CommandLine),
    File(FileLine)
}

enum CommandLine {
    Cd(String),
    Ls
}

enum FileLine {
    Dir(String),
    File(String, u32)
}

impl FromStr for FileSystemOutputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "$" => {
              let command = s.parse::<CommandLine>().expect("Could not parse command");
              Ok(FileSystemOutputLine::Command(command))
            }
            _ => {
              let file = s.parse::<FileLine>().expect("Could not parse file");
              Ok(FileSystemOutputLine::File(file))
            }

        }
    }
}

impl FromStr for CommandLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[2..4] {
            "cd" => Ok(CommandLine::Cd(s[5..].to_string())),
            "ls" => Ok(CommandLine::Ls),
            _ => unreachable!()
        }
    }
}

impl FromStr for FileLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let (dir_or_size, name) = (split.next().unwrap(), split.next().unwrap());

        match dir_or_size {
            "dir" => Ok(FileLine::Dir(name.to_string())),
            _ => Ok(FileLine::File(name.to_string(), dir_or_size.parse::<u32>().unwrap()))

        }
    }
}
