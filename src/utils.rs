use std::{path::Path, fs, io};

pub fn read_file_to_array<P: AsRef<Path>>(path: P, separator: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;

    let array = content
        .split(separator)
        .map(|s| s.to_string())
        .collect();

    Ok(array)
}


pub fn read_file_to_2d_array<P: AsRef<Path>>(path: P, separator: &str) -> io::Result<Vec<Vec<i32>>> {
    let content = fs::read_to_string(path)?;

    let array = content
        .split(separator)
        .map(|section| section.split('\n').filter(|l| !l.is_empty()).map(|str| str.parse::<i32>().unwrap()).collect())
        .collect();

    Ok(array)
}