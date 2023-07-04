use std::collections::HashSet;

use crate::utils;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Coordinate{
    x : i32,
    y : i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

struct Instruction{
    direction : Direction,
    distance : usize
}

fn char_to_direction(ch : char) -> Direction {
    match ch {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction")
    }
}

fn string_to_instruction(s : &str) -> Instruction {
    let direction = char_to_direction(s.chars().nth(0).unwrap());
    let distance = s.chars().skip(2).collect::<String>().parse::<usize>().unwrap();
    Instruction{direction, distance}
}

fn is_adjacent(c1 : Coordinate, c2 : Coordinate) -> bool {
    let x_diff = (c1.x - c2.x).abs();
    let y_diff = (c1.y - c2.y).abs();
    x_diff <= 1 && y_diff <= 1
}

pub fn part1(){
    let instructions = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/9.txt", "\n")
        .unwrap();

    let instruction_iter = instructions
        .iter()
        .map(|s| string_to_instruction(s));

    let mut tail_coordinates : Vec<Coordinate> = Vec::new();

    let mut head = Coordinate{x:0, y:0};
    let mut current_tail = head.clone();
    tail_coordinates.push(current_tail.clone());

    for instruction in instruction_iter{
        for _ in 0..instruction.distance{
            head = get_new_head(head, instruction.direction);
            
            let new_tail = get_new_tail(current_tail, head, instruction.direction);

            if new_tail != current_tail{
                current_tail = new_tail;
                tail_coordinates.push(current_tail.clone());
            }
        }
    }

    println!("Part 1: {:?}", tail_coordinates.iter().collect::<HashSet<_>>().len());
}

fn get_new_tail(current_tail: Coordinate, new_head: Coordinate, direction: Direction) -> Coordinate{
    if is_adjacent(current_tail, new_head){
        return current_tail;
    }

    let mut aligned_tail = align_tail(current_tail, new_head, direction);

    match direction {
        Direction::Up => aligned_tail.y += 1,
        Direction::Down => aligned_tail.y -= 1,
        Direction::Left => aligned_tail.x -= 1,
        Direction::Right => aligned_tail.x += 1
    }
    aligned_tail
}

fn align_tail(tail: Coordinate, head: Coordinate, moved: Direction) -> Coordinate{
    if !is_diagonal(tail, head){
        return tail;
    }

    let mut new_tail = tail.clone();

    if moved == Direction::Up || moved == Direction::Down{
        new_tail.x = head.x;
    }
    else{
        new_tail.y = head.y;
    }

    new_tail
}

fn is_diagonal(c1 : Coordinate, c2 : Coordinate) -> bool {
    c1.x != c2.x && c1.y != c2.y
}

fn get_new_head(current : Coordinate, direction: Direction) -> Coordinate{
    match direction {
        Direction::Up => Coordinate{x: current.x, y: current.y + 1},
        Direction::Down => Coordinate{x: current.x, y: current.y - 1 as i32},
        Direction::Left => Coordinate{x: current.x - 1 as i32, y: current.y},
        Direction::Right => Coordinate{x: current.x + 1 as i32, y: current.y}
    }
}