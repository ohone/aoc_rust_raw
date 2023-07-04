use crate::utils;

pub fn part1(){
    let bags = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/3.txt", "\n")
        .unwrap();

    let mut result = 0;

    for bag in bags{
        let length = bag.len();
        let compartment1 = &bag[0..length/2];
        let compartment2 = &bag[length/2..length];
        let common_char = get_common_char(compartment1, compartment2);
        result += char_to_val(common_char.unwrap());
    }

    println!("Result: {}", result);
}

pub fn part2(){
    let bags = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/3.txt", "\n")
        .unwrap();

    let mut result = 0;

    for n in 0..(bags.len() / 3){
        let group_index = n * 3;
        let group = &bags[group_index..=group_index+2];

        let common_chars = get_common_chars(&group[0], &group[1]);
        
        for char in group[2].chars(){
            if common_chars.contains(&char){
                result += char_to_val(char);
                break;
            }
        }
    }

    println!("Result: {}", result);
}

fn char_to_val(char: char) -> i32 {
    let c = char as i32;
    return if c > 96 { c - 96 } else { c - 38 };
}

fn get_common_char(str1: &str, str2: &str) -> Option<char> {
    for char in str1.chars(){
        for comp_char in str2.chars(){
            if char == comp_char {
                return Some(char);
            }
        }
    }

    return None;
}

fn get_common_chars(str1: &str, str2: &str) -> Vec<char> {
    let mut result = Vec::new();
    for char in str1.chars(){
        for comp_char in str2.chars(){
            if char == comp_char {
                result.push(char);
            }
        }
    }

    return result;
}