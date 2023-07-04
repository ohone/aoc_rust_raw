use crate::utils;

#[derive(Debug)]
struct Directory{
    directories: Vec<Directory>,
    files: Vec<File>
}

#[derive(Debug)]
struct File{
    size: i32
}

pub fn part1(){
    let lines = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/7.txt", "\n")
        .unwrap();

    let dir = poplate_directory("/", &mut lines.iter().skip(1));
    println!("{:?}", dir);
    let result = dirs_smaller_than(dir, 100000);
    println!("Result: {}", result);
}

pub fn part2(){
    let lines = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/7.txt", "\n")
    .unwrap();

    let dir = poplate_directory("/", &mut lines.iter().skip(1));

    let fs_used = dir_size(&dir);

    const FS_SIZE: i32 = 70000000;
    const UPDATE_SIZE : i32 = 30000000; 

    let size_req = UPDATE_SIZE - (FS_SIZE - fs_used);
    println!("{:?}", dir);
    let result = dirs_larger_than(dir, size_req);
    let min = result.iter().min().unwrap();
    println!("Result: {}", min); 
}

fn dirs_larger_than(root: Directory, size: i32) -> Vec<i32>{
    let mut accum = Vec::new();
    let root_size = dir_size(&root);
    if root_size > size{
        accum.push(root_size);
    }
    for dirs in root.directories{
        accum.append(&mut dirs_larger_than(dirs, size));
    }

    return accum;
}

fn dirs_smaller_than(dir : Directory, size: i32) -> i32{
    let mut accum = 0;
    let root_size = dir_size(&dir);
    if (root_size <= size){
        accum += root_size;
    }
    for dirs in dir.directories{
        accum += dirs_smaller_than(dirs, size);
    }

    return accum;
}

fn dir_size(dir: &Directory) -> i32 {
    let mut size = 0;
    for file in &dir.files{
        size += file.size;
    }
    for directory in &dir.directories{
        size += dir_size(&directory);
    }
    size
}

// ls enters here
fn poplate_directory<'a, I>(name: &str, lines: &mut I) -> Directory where I: Iterator<Item = &'a String> {
    let mut current_dir = Directory{
        directories: Vec::new(),
        files: Vec::new()
    };

    while let Some(line) = lines.next(){
        let line_split = line.split(" ").collect::<Vec<&str>>();
        match line_split[0]{
            "$" => {
                match line_split[1]{
                    "ls" => {
                        // process ls ?
                    },
                    "cd" => {
                        match line_split[2]{
                            ".." => {
                                return current_dir;
                            },
                            _ => {
                                current_dir.directories.push(poplate_directory(line_split[2], lines));// process cd <dir>
                            }
                        }
                    },
                    _ => {panic!("Unknown command {}", line_split[1])}
                }
            },
            "dir" => {

            },
            _ => {
                current_dir.files.push(File{
                    size: line_split[0].parse::<i32>().unwrap()
                });
            }
        }
    }

    return current_dir;
}

