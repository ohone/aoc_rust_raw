use std::{collections::BinaryHeap, cmp::Reverse};

use crate::utils;

pub fn part1() {
    let strings = utils::read_file_to_2d_array("/Users/eoghan/repos/aoc_rust_raw/src/1.txt", "\n\n");
    match strings {
        Ok(bags) => {
            // use min heap to keep track of the 3 largest numbers
            // only ever need to remove the smallest
            let mut heap = BinaryHeap::new();
            for bag in bags{
                let bag_count : i32 = bag.iter().sum();
                if heap.len() < 3 {
                    // If we have less than 3 elements, just push
                    heap.push(Reverse(bag_count));
                } else {
                    // If we have 3 elements, check if the new number is larger
                    if bag_count > heap.peek().unwrap().0 {
                        // If it is, pop the smallest and push the new one
                        heap.pop();
                        heap.push(Reverse(bag_count));
                    }
                }
            }
            let mut sum = 0;
            while let Some(Reverse(num)) = heap.pop() {
                sum += num;
            }
            println!("Sum: {}", sum);
        }
        Err(error) => println!("Error: {}", error),
    }
}


pub fn part2() {
    let strings = utils::read_file_to_2d_array("/Users/eoghan/repos/aoc_rust_raw/src/1.txt", "\n\n");
    match strings {
        Ok(bags) => {
            let mut largest = 0;
            for bag in bags {
                let bag_count : i32 = bag.iter().sum();
                largest = if bag_count > largest { bag_count } else { largest };
            }
            println!("Largest: {}", largest);
        }
        Err(error) => println!("Error: {}", error),
    }
}
