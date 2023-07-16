use std::collections::VecDeque;

use crate::utils;

pub fn part1(){
    let round_count = 20;

    let mut monkeys = get_monkeys();
    let mut tallys : Vec<usize> = monkeys.iter().map(|_| 0).collect();

    // foreach round_count
    for _ in 0..round_count{
        // foreach monkey
        for j in 0..monkeys.len(){
            let movements = process_monkey(&monkeys[j], modify_worry_level, 3);
            tallys[j] = tallys[j] + movements.len();
            for movement in movements{
                {
                    let target_monkey = &mut monkeys[movement.to];
                    target_monkey.items.push_back(movement.value);
                    monkeys[j].items.pop_front();
                }
            }
        }
    }

   let two_largets = two_largest(tallys).unwrap();

    println!("Part 1: {:?}", two_largets.0 * two_largets.1);
}

pub fn part2(){
    let round_count = 10000;

    let mut monkeys = get_monkeys();
    let mut tallys : Vec<usize> = monkeys.iter().map(|_| 0).collect();
    let lcm = monkeys.iter().fold(1, |m: usize,l| m * l.test.divisor);
    // foreach round_count
    for _ in 0..round_count{
        // foreach monkey
        for j in 0..monkeys.len(){
            let movements = process_monkey(&monkeys[j], modify_worry_level, 1);
            tallys[j] = tallys[j] + movements.len();
            for movement in movements{
                {
                    let target_monkey = &mut monkeys[movement.to];
                    target_monkey.items.push_back(movement.value % lcm);
                    monkeys[j].items.pop_front();
                }
            }
        }
    }
    println!("tallys {:?}", tallys);

    let two_largets = two_largest(tallys).unwrap();

    println!("Part 2: {:?}", two_largets.0 * two_largets.1);
}

fn get_monkeys() -> Vec<Monkey>{
    let input : Vec<String> = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/11.txt", "\n\n")
        .unwrap();

    input
        .iter()
        .map(|s| {
            let mut lines =  s.split("\n").skip(1);

            let items = lines
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect::<VecDeque<usize>>();

            let operation = lines
                .next()
                .unwrap()
                .split("=")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .to_string();

            let test_divisor = lines
                .next()
                .unwrap()
                .split("by")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse::<usize>().unwrap();

            let passed_target = lines
                .next()
                .unwrap()
                .split("monkey")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            let failed_target = lines
                .next()
                .unwrap()
                .split("monkey")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            return Monkey{
                items,
                operation,
                test: Test{
                    divisor: test_divisor,
                    passed_target,
                    failed_target
                }
            }
        })
        .collect()
}

fn two_largest(mut nums: Vec<usize>) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }
    
    nums.sort_unstable_by(|a, b| b.cmp(a));
    Some((nums[0], nums[1]))
}

fn process_monkey(monkey: &Monkey, relief_modifier: fn(usize, &String, usize) -> usize, divisor: usize) -> Vec<Movement>{
    let mut movements = Vec::new();
    for k in 0..monkey.items.iter().len(){
        let relief_worry_level = relief_modifier(monkey.items[k], &monkey.operation, divisor);
        movements.push(Movement{
            value: relief_worry_level,
            to: run_test(relief_worry_level, monkey.test)
        });
    };

    return movements;
}

fn modify_worry_level(initial_worry: usize, operation: &String, divisor: usize) -> usize{
    run_operation(initial_worry, operation) / divisor
}

fn run_operation(old: usize, operation: &str) -> usize{
    let parts = operation.split(' ').skip(1).collect::<Vec<&str>>();
    let op: fn(usize,usize) -> usize = match parts[0]{
        "+" => |a, b| a + b,
        "-" => |a, b| a - b,
        "*" => |a, b| a * b,
        "/" => |a, b| a / b,
        _ => panic!("Unknown operation")
    };

    let operand = match parts[1].parse::<usize>(){
        Ok(v) => v,
        Err(_) => old
    };

    return op(old, operand);
}

fn run_test(old: usize, test: Test) -> usize{
    if old % test.divisor == 0{
        return test.passed_target;
    }
    return test.failed_target;
}

struct Movement{
    value: usize,
    to: usize
}

#[derive(Debug, Clone)]
struct Monkey{ 
    items: VecDeque<usize>,
    operation: String,
    test: Test,
}

#[derive(Debug, Clone, Copy)]
struct Test{
    divisor: usize,
    passed_target: usize,
    failed_target: usize
}