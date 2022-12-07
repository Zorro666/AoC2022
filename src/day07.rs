use crate::file_to_vec;

/*
--- Day 7: No Space Left On Device ---

You can hear birds chirping and raindrops hitting leaves as the expedition proceeds.
Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system.
You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device
Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input).
For example:

$ cd /
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
7214296 k
The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files).
The outermost directory is called /.
You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

cd means change directory.
This changes which directory is the current directory, but the specific result depends on the argument:
cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
cd / switches the current directory to the outermost directory, /.
ls means list.
It prints out all of the files and directories immediately contained by the current directory:
123 abc means that the current directory contains a file named abc with size 123.
dir xyz means that the current directory contains a directory named xyz.
Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a).
 These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion.
To do this, you need to determine the total size of each directory.
The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly.
 (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
Directory d has total size 24933642.
As the outermost directory, / contains every file.
Its total size is 48381165, the sum of the size of every file.
To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes.
In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584).
(As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000.

What is the sum of the total sizes of those directories?

Your puzzle answer was 1243729.

--- Part Two ---

Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

Delete directory e, which would increase unused space by 584.
Delete directory a, which would increase unused space by 94853.
Delete directory d, which would increase unused space by 24933642.
Delete directory /, which would increase unused space by 48381165.
Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?

*/

static INPUT_FILE: &str = "data/day07/input.txt";

pub fn run() {
    println!("Day07: Start");
    Day::instance(true).execute();
    Day::instance(false).execute();
    println!("Day07: End");
}

struct Day {
    part1: bool,
    dir_dir_totals: Vec<i64>,
}

impl Day {
    fn instance(part1: bool) -> Day {
        Day {
            part1: part1,
            dir_dir_totals: Vec::new(),
        }
    }

    fn execute(&mut self) {
        let lines = file_to_vec(INPUT_FILE).expect("Could not load file");
        self.parse(&lines);

        if self.part1 {
            let result1 = self.sum_directories(100000);
            println!("Day07: Result1 {result1}");
            let expected = 1243729;
            if result1 != expected {
                panic!("Part1 is broken {result1} != {expected}");
            }
        } else {
            let result2 = self.find_smallest(70000000, 30000000);
            println!("Day07: Result2 {result2}");
            let expected = 4443914;
            if result2 != expected {
                panic!("Part2 is broken {result2} != {expected}");
            }
        }
    }

    fn parse(&mut self, lines: &Vec<String>) {
        let mut dir_parents: Vec<usize> = Vec::new();
        let mut dir_files: Vec<Vec<(&str, i64)>> = Vec::new();
        let mut dir_dirs: Vec<Vec<(&str, usize)>> = Vec::new();
        let mut dir_file_totals: Vec<i64> = Vec::new();
        let mut current_dir = 0;
        dir_parents.push(std::usize::MAX);
        dir_files.push(Vec::new());
        dir_dirs.push(Vec::new());
        dir_file_totals.push(0);
        self.dir_dir_totals.push(0);
        for line in lines {
            // lines begin with $ are commands you executed, very much like some modern computers:
            if line.starts_with("$ cd") {
                // cd x : look in current directory for the directory x and make it current directory.
                // cd .. : moves up one level
                // cd / : switches the current directory to the outermost directory, /.
                let dir_name = &line[5..];
                if dir_name == "/" {
                    current_dir = 0;
                    continue;
                }
                if dir_name == ".." {
                    current_dir = dir_parents[current_dir];
                    continue;
                }
                let mut found = false;
                for sd in &dir_dirs[current_dir] {
                    if sd.0 == dir_name {
                        current_dir = sd.1;
                        found = true;
                        break;
                    }
                }
                assert!(found);
                continue;
            } else if line.starts_with("$ ls") {
                // ls : lists all files and directories immediately contained by current directory:
                continue;
            }
            if line.starts_with("dir ") {
                // dir xyz : directory contains a directory named xyz.
                let dir_name = &line[4..];
                let mut dir_index = std::usize::MAX;
                for sd in &dir_dirs[current_dir] {
                    if sd.0 == dir_name {
                        dir_index = sd.1;
                        break;
                    }
                }
                if dir_index == std::usize::MAX {
                    dir_index = dir_parents.len();
                    dir_parents.push(current_dir);
                    dir_files.push(Vec::new());
                    dir_dirs.push(Vec::new());
                    dir_file_totals.push(0);
                    self.dir_dir_totals.push(0);
                    dir_dirs[current_dir].push((dir_name, dir_index));
                }
                continue;
            }
            // 123 abc : directory contains a file named abc with size 123.
            let toks: Vec<&str> = line.split(" ").collect();
            let file_size = toks[0].parse().expect("Not a number");
            let file_name = toks[1];
            let mut found = false;
            for sd in &dir_files[current_dir] {
                if sd.0 == file_name {
                    found = true;
                    break;
                }
            }
            if !found {
                dir_files[current_dir].push((file_name, file_size));
                dir_file_totals[current_dir] += file_size;
            }
            continue;
        }
        for d in 0..dir_parents.len() {
            let mut sub_dir_total = 0;
            let mut subdirs = Vec::new();
            subdirs.push(d);
            while !subdirs.is_empty() {
                let dir = subdirs.pop().unwrap();
                sub_dir_total += dir_file_totals[dir];
                for i in 0..dir_dirs[dir].len() {
                    let subdir_index = dir_dirs[dir][i].1;
                    subdirs.push(subdir_index);
                }
            }
            self.dir_dir_totals[d] = sub_dir_total;
        }
    }

    fn sum_directories(&self, max_dir_size: i64) -> i64 {
        let mut total = 0_i64;
        for v in &self.dir_dir_totals {
            if *v <= max_dir_size {
                total += v;
            }
        }
        return total;
    }

    fn find_smallest(&self, disk_size: i64, update_size: i64) -> i64 {
        let mut min_to_delete = std::i64::MAX;
        let min_unused_space = disk_size - self.dir_dir_totals[0];
        for v in &self.dir_dir_totals {
            if min_unused_space + v >= update_size {
                min_to_delete = std::cmp::min(min_to_delete, *v);
            }
        }
        return min_to_delete;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_array_to_string_array;

    #[test]
    fn part1() {
        let input: (Vec<&str>, i64) = (
            vec![
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
                "7214296 k",
            ],
            95437,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.sum_directories(100000), input.1);
    }

    #[test]
    fn part2() {
        let input: (Vec<&str>, i64) = (
            vec![
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
                "7214296 k",
            ],
            24933642,
        );
        let lines = str_array_to_string_array(input.0);
        let mut day = Day::instance(true);
        day.parse(&lines);
        assert_eq!(day.find_smallest(70000000, 30000000), input.1);
    }
}
