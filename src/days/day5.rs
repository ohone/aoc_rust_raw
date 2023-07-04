use std::fs;

#[derive(Debug)]
struct Instruction{
    count: i32,
    source: i32,
    dest: i32
}

pub fn part1(){
    let info = fs::read_to_string("/Users/eoghan/repos/aoc_rust_raw/src/5.txt").unwrap();

    let parts = info.split("\n\n").collect::<Vec<&str>>();

    let mut stacks = parse_stacks(parts[0]);

    parse_moves(parts[1]).for_each(|instruction| {
        for _ in 0..instruction.count{
            let moved_char = (stacks[(instruction.source - 1) as usize]).pop().unwrap();
            let dest_stack = &mut stacks[(instruction.dest - 1) as usize];
            dest_stack.push(moved_char);
        }
    });

    let result = stacks.iter().map(|s| s[s.len() - 1]).collect::<String>();
    println!("Result: {}", result);
}

fn parse_stacks(stack_string: &str) -> Vec<Vec<char>>{
    let stack_layer_strings = stack_string.split("\n").collect::<Vec<&str>>();
    let mut stacks = stack_layer_strings[stack_layer_strings.len() - 1]
        .split_whitespace()
        .map(|_| Vec::<char>::new())
        .collect::<Vec<Vec<char>>>();

    for mut stack_str in stack_layer_strings.iter().rev().skip(1).map(|s| s.chars()){
        for i in 0..stacks.len() {
            let index = if i == 0 { 1 } else { 3 }; 
            match stack_str.nth(index){
                Some(c) => { if !c.is_whitespace() { stacks[i].push(c) }},
                None => print!("error {:?}", stack_str.clone().collect::<String>())   
            }
        }
    };

    return stacks;
}

fn parse_moves<'a>(move_string: &'a str) -> impl Iterator<Item = Instruction> + 'a{
    return move_string.split("\n") 
        .map(|s| {
            let instruction_parts = s.split(' ').collect::<Vec<&str>>();
            let amount = instruction_parts[1].parse::<i32>().unwrap();
            let source = instruction_parts[3].parse::<i32>().unwrap();
            let destination = instruction_parts[5].parse::<i32>().unwrap();
            return Instruction{count: amount, source: source, dest: destination};
        });
}