use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone, Eq, PartialEq, Default)]
struct FileTree {
    files: Vec<(String, u32)>,
    dirs: Vec<(String, Rc<RefCell<FileTree>>)>,
    parent: Option<Rc<RefCell<FileTree>>>,
}

impl Debug for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileTree")
            .field("files", &self.files)
            .field("dirs", &self.dirs)
            .field("parent_linked", &self.parent.is_some())
            .finish()
    }
}

fn parse_input(
    input: &[&str],
    current_dir: Option<Rc<RefCell<FileTree>>>,
) -> Option<Rc<RefCell<FileTree>>> {
    if input.is_empty() {
        return current_dir;
    }
    let current = input[0];

    let mut lines = current.lines();
    let command = lines.next().expect("No command");
    let command_output = lines.collect::<Vec<_>>();

    let (command, payload) = if command.find(' ').is_some() {
        command.split_once(' ').unwrap()
    } else {
        (command, "")
    };

    match command {
        "ls" => {
            let mut files = vec![];
            let mut dirs = vec![];

            for o in command_output {
                let (size_or_dir, name) =
                    o.split_once(' ').expect("No separator in command output");

                if size_or_dir == "dir" {
                    let file_tree = FileTree {
                        parent: current_dir.clone(),
                        ..Default::default()
                    };
                    dirs.push((name.to_string(), Rc::new(RefCell::new(file_tree))));
                } else {
                    let size: u32 = size_or_dir.parse().expect("Could not parse file size");
                    files.push((name.to_string(), size));
                }
            }

            let cur_ls = if let Some(dir) = current_dir {
                let mut tree = dir.as_ref().borrow_mut();
                tree.files.append(&mut files);
                tree.files.dedup();
                tree.dirs.append(&mut dirs);
                tree.dirs.dedup();
                dir.clone()
            } else {
                Rc::new(RefCell::new(FileTree {
                    files,
                    dirs,
                    parent: None,
                }))
            };
            parse_input(&input[1..input.len()], Some(cur_ls))
        }
        "cd" => {
            if payload != ".." {
                if let Some(dirw) = current_dir {
                    let cur_cd = {
                        let mut dir = dirw.as_ref().borrow_mut();
                        let cur_cd = if let Some((_, tree)) =
                            dir.dirs.iter().cloned().find(|(name, _)| name == payload)
                        {
                            tree
                        } else {
                            let new_dir = Rc::new(RefCell::new(FileTree {
                                files: vec![],
                                dirs: vec![],
                                parent: Some(dirw.clone()),
                            }));
                            dir.dirs.push((payload.to_string(), new_dir.clone()));
                            new_dir
                        };

                        cur_cd
                    };

                    parse_input(&input[1..input.len()], Some(cur_cd))
                } else {
                    let cur_cd_2 = Rc::new(RefCell::new(FileTree {
                        parent: None,
                        ..Default::default()
                    }));
                    let tree = Rc::new(RefCell::new(FileTree {
                        files: vec![],
                        dirs: vec![(payload.to_string(), cur_cd_2.clone())],
                        parent: None,
                    }));
                    let mut t = cur_cd_2.as_ref().borrow_mut();
                    t.parent = Some(tree);
                    parse_input(&input[1..input.len()], Some(cur_cd_2.clone()))
                }
            } else {
                let cur_cd_3 = if let Some(dir) = current_dir {
                    let dir = dir.as_ref().borrow();
                    dir.parent.clone()
                } else {
                    current_dir
                };
                parse_input(&input[1..input.len()], cur_cd_3)
            }
        }
        _ => panic!("Unexpected command '{command}'"),
    }
}

fn get_root(mut file_tree: FileTree) -> FileTree {
    while let Some(parent) = file_tree.parent {
        file_tree = parent.as_ref().borrow().clone();
    }
    file_tree
}

fn get_dir_size(root_fs: FileTree) -> u64 {
    let file_sizes = root_fs
        .files
        .iter()
        .map(|(_name, size)| *size as u64)
        .sum::<u64>();
    let dirs_sizes = root_fs
        .dirs
        .iter()
        .map(|(_name, tree)| get_dir_size(tree.as_ref().borrow().clone()))
        .sum::<u64>();

    file_sizes + dirs_sizes
}

fn part1(root_fs: FileTree, cur: &mut u64) -> &mut u64 {
    for (_dirname, tree) in root_fs.dirs {
        let dir_tree = tree.as_ref().borrow().clone();
        let dir_size = get_dir_size(dir_tree.clone());
        if dir_size <= 100000 {
            *cur += dir_size;
        }
        part1(dir_tree, cur);
    }
    cur
}

fn part2(root_fs: FileTree, min_dir_size: u64, cur: &mut (String, u64)) -> &mut (String, u64) {
    for (dirname, tree) in root_fs.dirs {
        let dir_tree = tree.as_ref().borrow().clone();
        let dir_size = get_dir_size(dir_tree.clone());
        if dir_size >= min_dir_size && dir_size < cur.1 {
            *cur = (dirname, dir_size);
        }
        part2(dir_tree, min_dir_size, cur);
    }
    cur
}

fn main() {
    let input = include_str!("day7.txt")
        .split('$')
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();

    let first_3 = &input[1..input.len()];

    let fs = parse_input(first_3, Some(Default::default())).expect("Expect FS");
    let root_fs = get_root(fs.as_ref().borrow().clone());

    let mut part1_size = 0_u64;
    part1(root_fs.clone(), &mut part1_size);
    println!("part 1 : {part1_size}");

    let dirs_size = get_dir_size(root_fs.clone());
    let min_dir_size = dirs_size - 40000000;
    let mut part2_dir = (":(".to_string(), u64::MAX);
    part2(root_fs, min_dir_size, &mut part2_dir);

    println!("part 2 : {part2_dir:?} (min : {min_dir_size})");
}
