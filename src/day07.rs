use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{bail, Context, Result};

use crate::read_lines;

fn construct_fs<I>(lines: I) -> Result<(HashSet<PathBuf>, HashMap<PathBuf, u32>)>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut fs: HashMap<_, u32> = HashMap::new();
    let mut all_dirs = HashSet::new();
    let mut current_path = PathBuf::new();
    for line in lines {
        let line = line.as_ref();
        if line.starts_with('$') {
            let command = &line[2..];
            let command_parts: Vec<_> = command.split(' ').collect();
            match command_parts.as_slice() {
                ["ls"] => {}
                ["cd", dir] => {
                    //reading_contents_of = None;
                    match *dir {
                        "/" => {
                            current_path = PathBuf::from("/");
                        }
                        ".." => {
                            current_path.pop();
                        }
                        subdir => {
                            current_path.push(subdir);
                        }
                    }
                    all_dirs.insert(current_path.clone());
                }
                _ => bail!("Command not found"),
            }
        } else {
            let ls_parts: Vec<_> = line.split(' ').collect();
            match ls_parts.as_slice() {
                ["dir", _] => {}
                [size, name] => {
                    let filename = current_path.as_path().join(name);
                    fs.insert(filename, size.parse()?);
                }
                _ => bail!("ls format not recognised"),
            }
        }
    }
    Ok((all_dirs, fs))
}

fn sum_small_dirs<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let (all_dirs, fs) = construct_fs(lines)?;
    let dir_sizes = all_dirs.iter().map(|d| {
        (
            d,
            fs.iter()
                .filter(|(p, _)| p.starts_with(d))
                .map(|(_, s)| s)
                .sum::<u32>(),
        )
    });
    let small_dirs: Vec<(_, _)> = dir_sizes.filter(|(_, s)| s <= &100000).collect();
    Ok(small_dirs.iter().map(|(_, s)| s).sum())
}

fn find_freeing_dir<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let (all_dirs, fs) = construct_fs(lines)?;
    let dir_sizes = all_dirs.iter().map(|d| {
        (
            d,
            fs.iter()
                .filter(|(p, _)| p.starts_with(d))
                .map(|(_, s)| s)
                .sum::<u32>(),
        )
    });
    let dir_sizes: HashMap<_, _> = dir_sizes.into_iter().collect();
    let total_space = 70000000;
    let required = 30000000;
    let currently_used: u32 = dir_sizes[&PathBuf::from("/")];
    let currently_free = total_space - currently_used;
    let deficit = required - currently_free;
    let freeing_dir = dir_sizes
        .iter()
        .filter(|(_, s)| s >= &&deficit)
        .min_by(|(_, s), (_, s_other)| s.cmp(s_other))
        .context("finding min dir to free space")?;
    Ok(*freeing_dir.1)
}

pub fn day7() -> Result<(u32, u32)> {
    let a = sum_small_dirs(read_lines!("day07.txt")).context("Finding smallest dirs")?;
    let b = find_freeing_dir(read_lines!("day07.txt")).context("Finding dir to free space")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day7() -> Result<()> {
        assert_eq!(
            sum_small_dirs(vec![
                "$ cd /",
                "$ ls",
                "dir a",
                "14848514 b.txt",
                "8504156 c.dat",
                "dir d",
                "$ cd a",
                "$ ls",
                "dir e",
                "29116 f",
                "2557 g",
                "62596 h.lst",
                "$ cd e",
                "$ ls",
                "584 i",
                "$ cd ..",
                "$ cd ..",
                "$ cd d",
                "$ ls",
                "4060174 j",
                "8033020 d.log",
                "5626152 d.ext",
                "7214296 k"
            ])?,
            95437
        );
        assert_eq!(
            find_freeing_dir(vec![
                "$ cd /",
                "$ ls",
                "dir a",
                "14848514 b.txt",
                "8504156 c.dat",
                "dir d",
                "$ cd a",
                "$ ls",
                "dir e",
                "29116 f",
                "2557 g",
                "62596 h.lst",
                "$ cd e",
                "$ ls",
                "584 i",
                "$ cd ..",
                "$ cd ..",
                "$ cd d",
                "$ ls",
                "4060174 j",
                "8033020 d.log",
                "5626152 d.ext",
                "7214296 k"
            ])?,
            24933642
        );
        assert_eq!(day7()?, (1232307, 7268994));
        Ok(())
    }
}
