use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    fmt::Write,
    rc::{Rc, Weak},
    task::Wake,
};

#[derive(Debug)]
struct FileSystem {
    root: Rc<RefCell<Directory>>,
}

impl FileSystem {
    fn new() -> Self {
        let root_dir = Directory {
            name: "/".to_owned(),
            children: HashMap::new(),
            parent: None,
        };
        Self {
            root: Rc::new(RefCell::new(root_dir)),
        }
    }
}

#[derive(Debug)]
enum DirectoryEntry {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: HashMap<String, DirectoryEntry>,
    parent: Option<Weak<RefCell<Directory>>>,
}

impl Directory {
    fn size(&self) -> u64 {
        self.children
            .iter()
            .map(|(_, child)| match child {
                DirectoryEntry::Directory(dir) => dir.as_ref().borrow().size(),
                DirectoryEntry::File(file) => file.size,
            })
            .sum()
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
    parent: Option<Weak<RefCell<Directory>>>,
}

#[derive(Debug)]
struct Shell {
    file_system: FileSystem,
    cwd: Rc<RefCell<Directory>>,
}

impl Shell {
    fn new(file_system: FileSystem) -> Self {
        let file_system = FileSystem::new();
        let cwd = file_system.root.clone();
        Self { file_system, cwd }
    }
    fn execute(&mut self, command: &Command) {
        match command.name {
            Commands::Cd => self.cd(&command),
            Commands::Ls => self.ls(&command),
        }
    }

    fn cd(&mut self, command: &Command) {
        match command.arg.as_str() {
            ".." => {
                let parent = match self.cwd.as_ref().borrow_mut().parent {
                    Some(ref parent) => parent.clone(),
                    None => return,
                };

                let parent = match parent.upgrade() {
                    Some(parent) => parent,
                    None => panic!("Failed to upgarde parent."),
                };

                self.cwd = parent;
            }
            arg => {
                // Already at cwd target directory
                if arg == self.cwd.borrow().name.as_str() {
                    return;
                }

                // Change directory to root
                if arg == "/" {
                    self.cwd = self.file_system.root.clone();
                }

                self.cwd = {
                    let mut borrowed = self.cwd.as_ref().borrow_mut();

                    let weak_parent = Rc::downgrade(&self.cwd);
                    let entry = borrowed.children.entry(arg.to_owned()).or_insert_with(|| {
                        // Create new directory if doesn't exist
                        let new_dir = Directory {
                            name: arg.to_owned(),
                            children: HashMap::new(),
                            parent: Some(weak_parent),
                        };

                        let new_dir = Rc::new(RefCell::new(new_dir));
                        let new_entry = DirectoryEntry::Directory(new_dir.clone());

                        new_entry
                    });

                    match entry {
                        DirectoryEntry::Directory(dir) => dir.clone(),
                        DirectoryEntry::File(file) => {
                            panic!("{} is not a directory.", file.name)
                        }
                    }
                }
            }
        }
    }

    fn ls(&mut self, command: &Command) {
        for line in &command.output {
            let (first_half, name) = line
                .split_once(' ')
                .unwrap_or_else(|| panic!("Invalid `ls` output: {}", line));
            let new_entry = if !self.cwd.borrow().children.contains_key(name) {
                // If can parse the first half, then is a file
                Some(if let Ok(file_size) = first_half.parse::<u64>() {
                    DirectoryEntry::File(File {
                        name: name.to_owned(),
                        size: file_size,
                        parent: Some(Rc::downgrade(&self.cwd)),
                    })
                } else {
                    DirectoryEntry::Directory(Rc::new(RefCell::new(Directory {
                        name: name.to_owned(),
                        children: HashMap::new(),
                        parent: Some(Rc::downgrade(&self.cwd)),
                    })))
                })
            } else {
                None
            };

            if let Some(new_entry) = new_entry {
                self.cwd
                    .as_ref()
                    .borrow_mut()
                    .children
                    .insert(name.to_owned(), new_entry);
            }
        }
    }
}

#[derive(Debug)]
struct Command {
    name: Commands,
    arg: String,
    output: Vec<String>,
}

#[derive(Debug)]
enum Commands {
    Cd,
    Ls,
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('$') {
            return Err("Invalid formatting. Command should start with a `$`".to_owned());
        }

        let (_, command_str) = value.split_once(' ').ok_or_else(|| "Command incomplete.")?;
        let (command, arg) = command_str.split_once(' ').unwrap_or((command_str, ""));

        let command = match command {
            "ls" => Commands::Ls,
            "cd" => Commands::Cd,
            _ => return Err(format!("Invalid command: {}", command)),
        };

        Ok(Self {
            name: command,
            arg: arg.to_owned(),
            output: Vec::new(),
        })
    }
}

pub fn solve_part1(input: &str) -> u64 {
    let cmds = parse_cmds(input);
    let file_system = FileSystem::new();
    let mut shell = Shell::new(file_system);

    for cmd in cmds {
        shell.execute(&cmd);
    }

    let root_dir = shell.file_system.root.borrow();
    root_dir.children.iter().filter_map(directory_filter).sum()
}

fn directory_filter(file_entry: (&String, &DirectoryEntry)) -> Option<u64> {
    let (_name, entry) = file_entry;

    match entry {
        DirectoryEntry::Directory(dir) => {
            let mut result_size = dir
                .borrow()
                .children
                .iter()
                .filter_map(directory_filter)
                .sum::<u64>();
            if dir.borrow().size() <= 100000 {
                result_size += dir.borrow().size();
            }

            Some(result_size)
        }
        _ => None,
    }
}

pub fn solve_part2(input: &str) -> usize {
    todo!()
}

fn parse_cmds(input: &str) -> Vec<Command> {
    let mut cmds: Vec<Command> = Vec::new();
    let mut output_buf = None;
    for line in input.lines() {
        let cmd = match Command::try_from(line) {
            Ok(cmd) => {
                match cmd.name {
                    Commands::Ls => {
                        let _ = output_buf.insert(String::new());
                    }
                    Commands::Cd => {
                        if let Some(ls) = cmds.last_mut() {
                            // take output_buf value and insert into last command parsed which
                            // should be a `Commands::Ls`
                            ls.output.extend(
                                output_buf
                                    .take()
                                    .unwrap_or(String::new())
                                    .lines()
                                    .map(|s| s.to_owned()),
                            )
                        }
                    }
                }
                cmd
            }
            Err(_) => match output_buf {
                Some(ref mut buf) => {
                    buf.write_fmt(format_args!("{line}\n")).unwrap();
                    continue;
                }
                None => panic!("Failed to parse line: {}", line),
            },
        };

        cmds.push(cmd);
    }

    if let Some(cmd) = cmds.last_mut() {
        if let Commands::Ls = cmd.name {
            cmd.output.extend(
                output_buf
                    .take()
                    .unwrap_or(String::new())
                    .lines()
                    .map(|s| s.to_owned()),
            )
        }
    }

    cmds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let input = r#"$ cd /
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

        assert_eq!(solve_part1(input), 95437);
    }

    #[test]
    fn can_solve_part2() {
        unimplemented!();
    }
}
