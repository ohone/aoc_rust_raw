use crate::utils;

pub fn part1(){
    let mut tally = 0;
    let strings = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/2.txt", "\n")
        .unwrap();

    // index : 0 = rock, 1 = paper, 2 = scissors, beats
    let beats = [2, 0, 1];

    let plays : Vec<Vec<&str>>  = strings
        .iter()
        .map(|s| (*s).split(' ').collect())
        .collect();

    for play in plays{
        let opp = (play[0].chars().nth(0).unwrap() as i32) - 65;
        let me = (play[1].chars().nth(0).unwrap() as i32) - 88;
        let win_amt = calculate_win_amt(opp, me, beats);
        tally += win_amt + me + 1;
    }

    println!("Tally: {}", tally);
}

pub fn part2(){
    let strings = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/2.txt", "\n")
        .unwrap();

    let lose_move_map = [2, 0, 1];
    let win_move_map = [1, 2, 0];
    let tally  = strings
        .iter()
        .map(|s| {
            let play = (*s).split(' ').collect::<Vec<&str>>() ;
            let opp = (play[0].chars().nth(0).unwrap() as i32) - 65;
            let outcome = (play[1].chars().nth(0).unwrap() as i32) - 88;
            let me = if outcome == 0 { lose_move_map[opp as usize] } else if outcome == 2 { win_move_map[opp as usize] } else { opp };
    
            return (outcome * 3) + me + 1;
        })
        .sum::<i32>();

    println!("Tally: {}", tally); 
}

fn calculate_win_amt(opp: i32, me: i32, beats: [i32; 3]) -> i32 {
    if opp == me {
        return 3;
    }
    if beats[opp as usize] == me {
        return 0;
    }
    return 6;
}