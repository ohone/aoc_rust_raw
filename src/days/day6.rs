use std::{fs};

fn find_distinct_buffer(size: usize, info: &Vec<char>) -> Option<usize>{
    let mut i = size - 1;
    while i < info.len() - 1{
        println!("looking for: {}", info[i]);
        let slice = &info[((i-(size - 1)))..i];
        println!("in slice: {:?}", slice);

        let position: Option<usize> = slice.iter()
            .enumerate()
            .filter(|&(_, &c)| c == info[i])
            .map(|(i, _)| i)
            .rev()
            .next();

        println!("found: {:?}", position);
        match position {
            Some(index) => {
                i += (index + size) - (size - 1);
                println!("skipping to index {}", i);
            },
            None => {
                let has_duplicates = {
                    let mut found = false;
                    for i in 0..slice.len() {
                        for j in i + 1..slice.len() {
                            if slice[i] == slice[j] {
                                found = true;
                                break;
                            }
                        }
                        if found { break; }
                    }
                    found
                };
                if has_duplicates {
                    i += 1;
                }
                else {
                    println!("{}", i + 1);
                    return Some(i + 1);
                }
            }
        }
    } 
    return None;
}

pub fn part1(){
    let info = fs::read_to_string("/Users/eoghan/repos/aoc_rust_raw/src/6.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let result = find_distinct_buffer(4, &info);
    println!("{:?}", result);
}

pub fn part2(){
    let info = fs::read_to_string("/Users/eoghan/repos/aoc_rust_raw/src/6.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let result = find_distinct_buffer(14, &info);
    println!("{:?}", result);
}