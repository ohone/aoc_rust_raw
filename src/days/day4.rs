use crate::utils;

struct Assignment {
    start: i32,
    end: i32
}

impl Assignment {
    fn from_string(s: &str) -> Result<Self, std::num::ParseIntError> {
        let parts: Vec<&str> = s.split('-').collect();
        let start = parts[0].trim().parse::<i32>()?;
        let end = parts[1].trim().parse::<i32>()?;
        Ok(Assignment { start, end })
    }

    fn envelops(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.start
    }
}

pub fn part1(){
    let elf_pairs : usize = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/4.txt", "\n")
        .unwrap()
        .iter()
        .map(|pair_string| {
            let mut assignments = pair_string.split(',');
            let assignment1 = Assignment::from_string(assignments.next().unwrap()).unwrap();
            let assignment2 = Assignment::from_string(assignments.next().unwrap()).unwrap();
            return (assignment1, assignment2);
        })
        .filter(|(a1, a2)| a1.envelops(a2) || a2.envelops(a1))
        .collect::<Vec<(Assignment, Assignment)>>()
        .len();

    println!("Tally: {}", elf_pairs);
}

pub fn part2(){
    let elf_pairs : usize = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/4.txt", "\n")
        .unwrap()
        .iter()
        .map(|pair_string| {
            let mut assignments = pair_string.split(',');
            let assignment1 = Assignment::from_string(assignments.next().unwrap()).unwrap();
            let assignment2 = Assignment::from_string(assignments.next().unwrap()).unwrap();
            return (assignment1, assignment2);
        })
        .filter(|(a1, a2)| a1.overlaps(a2) || a2.overlaps(a1))
        .collect::<Vec<(Assignment, Assignment)>>()
        .len();

    println!("Tally: {}", elf_pairs);
}