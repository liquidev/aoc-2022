use std::collections::HashMap;

use aoc::{
    anyhow::{self, anyhow, bail, Context},
    wrap_main, Challenge,
};

#[derive(Debug, Clone)]
enum EntryKind {
    File { size: usize },
    Directory { entries: HashMap<String, Inode> },
}

impl EntryKind {
    fn new_empty_directory() -> Self {
        Self::Directory {
            entries: HashMap::new(),
        }
    }

    #[must_use]
    fn is_directory(&self) -> bool {
        matches!(self, Self::Directory { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Inode(usize);

impl Inode {
    const ROOT: Inode = Inode(0);
}

#[derive(Debug, Clone)]
struct Entry {
    parent: Inode,
    name: String,
    kind: EntryKind,
}

#[derive(Debug, Clone)]
struct Filesystem {
    entries: Vec<Entry>,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            entries: vec![Entry {
                parent: Inode::ROOT,
                name: "".to_owned(),
                kind: EntryKind::new_empty_directory(),
            }],
        }
    }

    fn get(&self, inode: Inode) -> &Entry {
        &self.entries[inode.0]
    }

    fn get_mut(&mut self, inode: Inode) -> &mut Entry {
        &mut self.entries[inode.0]
    }

    fn create(&mut self, parent: Inode, name: String, kind: EntryKind) -> anyhow::Result<Inode> {
        let inode = Inode(self.entries.len());
        self.entries.push(Entry {
            parent,
            name: name.clone(),
            kind,
        });
        let entry = self.get_mut(parent);
        match &mut entry.kind {
            EntryKind::File { .. } => {
                bail!("parent is not a directory (parent {parent:?}, {entry:?})")
            }
            EntryKind::Directory { entries } => {
                entries.insert(name, inode);
                Ok(inode)
            }
        }
    }

    fn recursive_size(&self, inode: Inode) -> usize {
        match &self.get(inode).kind {
            EntryKind::File { size } => *size,
            EntryKind::Directory { entries } => entries
                .values()
                .map(|&inode| self.recursive_size(inode))
                .sum(),
        }
    }

    fn inodes(&self) -> impl Iterator<Item = (Inode, &Entry)> + '_ {
        self.entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (Inode(index), entry))
    }

    fn print_tree(&self, inode: Inode) {
        fn print_tree_recursively(filesystem: &Filesystem, inode: Inode, level: usize) {
            for _ in 0..level {
                print!("  ")
            }
            let entry = filesystem.get(inode);
            match &entry.kind {
                EntryKind::File { size } => println!("{size} {}", entry.name),
                EntryKind::Directory { entries } => {
                    println!(
                        "{}/ (total {})",
                        entry.name,
                        filesystem.recursive_size(inode)
                    );
                    for &inode in entries.values() {
                        print_tree_recursively(filesystem, inode, level + 1);
                    }
                }
            }
        }
        print_tree_recursively(self, inode, 0);
    }
}

#[derive(Debug, Clone)]
struct Shell {
    cwd: Inode,
}

impl Shell {
    fn new() -> Self {
        Self { cwd: Inode::ROOT }
    }

    fn enter_directory(&mut self, filesystem: &Filesystem, name: &str) -> anyhow::Result<()> {
        match name {
            "/" => {
                self.cwd = Inode::ROOT;
                Ok(())
            }
            ".." => {
                self.cwd = filesystem.get(self.cwd).parent;
                Ok(())
            }
            _ => match &filesystem.get(self.cwd).kind {
                EntryKind::File { .. } => bail!("{name} is a file and cannot be entered"),
                EntryKind::Directory { entries } => {
                    self.cwd = *entries
                        .get(name)
                        .ok_or_else(|| anyhow!("no file or directory named {name}"))?;
                    Ok(())
                }
            },
        }
    }
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let mut filesystem = Filesystem::new();
    let mut shell = Shell::new();

    for line in challenge.input.lines() {
        let mut words = line.split_whitespace();
        let kind = words.next().ok_or_else(|| {
            anyhow!("line is missing first word ('$', 'dir', or file size): {line}")
        })?;
        match kind {
            "$" => {
                let command = words
                    .next()
                    .ok_or_else(|| anyhow!("missing command: {line}"))?;
                match command {
                    "cd" => {
                        let name = words
                            .next()
                            .ok_or_else(|| anyhow!("missing path to cd to: {line}"))?;
                        shell.enter_directory(&filesystem, name)?;
                    }
                    "ls" => (),
                    _ => bail!("unknown command: {command}"),
                }
            }
            "dir" => {
                let directory_name = words
                    .next()
                    .ok_or_else(|| anyhow!("missing directory name: {line}"))?;
                filesystem
                    .create(
                        shell.cwd,
                        directory_name.to_owned(),
                        EntryKind::new_empty_directory(),
                    )
                    .with_context(|| format!("cannot create directory {directory_name}"))?;
            }
            file_size => {
                let file_name = words
                    .next()
                    .ok_or_else(|| anyhow!("missing file name after size: {line}"))?;
                let file_size = file_size.parse().context("cannot parse file size")?;
                filesystem
                    .create(
                        shell.cwd,
                        file_name.to_owned(),
                        EntryKind::File { size: file_size },
                    )
                    .with_context(|| format!("cannot create file {file_name}"))?;
            }
        }
    }

    filesystem.print_tree(Inode::ROOT);

    let size_sum: usize = filesystem
        .inodes()
        .filter_map(|(inode, entry)| {
            entry
                .kind
                .is_directory()
                .then(|| filesystem.recursive_size(inode))
        })
        .filter(|&size| size <= 100000)
        .sum();
    println!("part 1: {size_sum}");

    let used_space = filesystem.recursive_size(Inode::ROOT);
    let disk_size = 70000000;
    let unused_space = disk_size - used_space;
    let update_needs = 30000000;
    let smallest_to_delete = filesystem
        .inodes()
        .filter_map(|(inode, entry)| {
            entry
                .kind
                .is_directory()
                .then(|| filesystem.recursive_size(inode))
        })
        .filter(|&size| unused_space + size >= update_needs)
        .min()
        .ok_or_else(|| anyhow!("no directory suitable for deletion found"))?;
    println!("part 2: {smallest_to_delete}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
