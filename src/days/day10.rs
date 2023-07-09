use crate::utils;

pub fn part1(){
    let instructions = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/10.txt", "\n")
        .unwrap();
    
    let mut X: i32 = 1;
    let mut cycle = 1;
    let mut signal_strength = 0;
    for instruction in instructions{
        if &instruction[..4] == "addx"{
            let res = increment_cycle(cycle, X);
            cycle = res.1;
            signal_strength += res.0;
            let res = increment_cycle(cycle, X);
            cycle = res.1;
            signal_strength += res.0;
            X += instruction[5..].parse::<i32>().unwrap();
        }
        else{
            let res = increment_cycle(cycle, X);
            cycle = res.1;
            signal_strength += res.0;
        }
    }

    println!("Part 1: {}", signal_strength);
}

pub fn increment_cycle(cycle: i32, register: i32) -> (i32, i32){
    let signal = if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        cycle * register
    }
    else{
        0
    };

    return (signal, (cycle + 1));
}

pub fn part2(){
    let instructions = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/10.txt", "\n")
    .unwrap();

    let mut X: i32 = 1;
    let mut cycle = 1;
    for instruction in instructions{
        if &instruction[..4] == "addx"{
            draw(cycle, X);
            let res = increment_cycle(cycle, X);
            cycle = res.1;
            draw(cycle, X);
            let res = increment_cycle(cycle, X);
            cycle = res.1;
            X += instruction[5..].parse::<i32>().unwrap();
        }
        else{
            draw(cycle, X);
            let res = increment_cycle(cycle, X);
            cycle = res.1;
        }
    }
}

pub fn draw(cycle: i32, register: i32){
    let crt_pos = (cycle - 1) % 40;
    let diff = (register) - crt_pos;
    if diff.abs() > 1 {
        print!(".");
    }
    else{
        print!("#");
    }

    if (cycle) % 40 == 0{
        println!("");
    }
}
