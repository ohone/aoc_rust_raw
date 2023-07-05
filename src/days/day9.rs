use std::{collections::HashSet, clone, thread::current};

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
            
            let new_tail = get_new_tail(current_tail, head);

            if new_tail != current_tail{
                current_tail = new_tail;
                tail_coordinates.push(current_tail.clone());
            }
        }
    }

    println!("Part 1: {:?}", tail_coordinates.iter().collect::<HashSet<_>>().len());
}

pub fn part2(){
    let instructions = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/9.txt", "\n")
        .unwrap();

    let instruction_iter = instructions
        .iter()
        .map(|s| string_to_instruction(s));

    let mut tail_coordinates : Vec<Coordinate> = Vec::new();

    tail_coordinates.push( Coordinate{x:0, y:0} );

    let knot_indices = (0..10).collect::<Vec<usize>>();
    let mut knots : Vec<Coordinate> = knot_indices
        .iter()
        .map(|_| Coordinate{x:0, y:0})
        .collect();

    for instruction in instruction_iter{
        for _ in 0..instruction.distance{
            knots[0] = get_new_head(knots[0], instruction.direction);

            for coord_idx in knot_indices.iter().skip(1){
                println!("knots[*coord_idx]: {:?}", knots);
                knots[*coord_idx] = get_new_tail(knots[*coord_idx], knots[*coord_idx - 1]);
                if *coord_idx == 9{
                    tail_coordinates.push(knots[*coord_idx].clone())
                }
            }
        }
    }

    println!("Part 2: {:?}", tail_coordinates.iter().collect::<HashSet<_>>().len());
}

fn get_new_tail(current_tail: Coordinate, new_head: Coordinate) -> Coordinate{
    if is_adjacent(current_tail, new_head){
        return current_tail;
    }

    return if aligned_x(current_tail, new_head){
        bump_y(current_tail, new_head)
    }
    else if aligned_y(current_tail, new_head){

        bump_x(current_tail, new_head)
    }
    else{ // move diagonally
        bump_y(bump_x(current_tail, new_head), new_head)
    }
}

fn bump_y(current_tail: Coordinate, new_head: Coordinate) -> Coordinate{
    if current_tail.y > new_head.y{
        return Coordinate{x: current_tail.x, y: current_tail.y - 1};
    }
    else{
        return Coordinate{x: current_tail.x, y: current_tail.y + 1};
    }
}

fn bump_x(current_tail: Coordinate, new_head: Coordinate) -> Coordinate{
    if current_tail.x > new_head.x{
        return Coordinate{x: current_tail.x - 1, y: current_tail.y};
    }
    else{
        return Coordinate{x: current_tail.x + 1, y: current_tail.y};
    }
}

fn aligned_y(tail: Coordinate, head: Coordinate) -> bool{
    tail.y == head.y
}

fn aligned_x(tail: Coordinate, head: Coordinate) -> bool{
    tail.x == head.x
}

fn align_diagonal_axis(tail: Coordinate, head: Coordinate) -> Coordinate{
    if !is_diagonal(tail, head){
        return tail.clone();
    }

    let mut new_tail = tail.clone();

    if (new_tail.x - head.x).abs() == 2{
        new_tail.y = head.y;
    }
    else if (new_tail.y - head.y).abs() == 2{
        new_tail.x = head.x;    
    }
    else{
        println!("tail: {:?}, head: {:?}", tail, head);
        panic!("aaa");
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