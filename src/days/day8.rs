use crate::utils;

pub fn part1(){
    let mut lines = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/8.txt", "\n")
        .unwrap()
        .iter()
        .map(|s| s.chars().map(|c| (c.to_digit(10).unwrap() as i32, false)).collect::<Vec<(i32,bool)>>())
        .collect::<Vec<Vec<(i32,bool)>>>();

    // horizontal checks
    for y in 1..lines.len() - 1{
        // left to right
        let left = lines[y][0].0;
        let line = &mut lines[y].iter_mut();
        observe_line_via_range_2(line, left);
        // right to left
        let right = lines[y][lines[y].len() - 1].0;
        let backward_line = &mut lines[y].iter_mut().rev();
        observe_line_via_range_2(backward_line, right);
    }

    // vertical checks
    for x in 1..lines[0].len() - 1{
        // top to bottom
        let top = lines[0][x].0; 
        let mut line = lines.iter_mut().map(|l| &mut l[x]);
        observe_line_via_range_2(&mut line, top);
        // bottom to top
        let bottom = lines[lines.len() - 1][x].0;
        let backward_line = &mut lines.iter_mut().map(|l| &mut l[x]).rev();
        observe_line_via_range_2(backward_line, bottom);
    }

    let x = lines[0].len();
    let y = lines.len();
    
    let inner_visible = lines
        .iter()
        .skip(1)
        .take(y - 2)
        .flat_map(|l| l.iter().skip(1).take(x - 2))
        .filter(|a| a.1)
        .count();
    let outer_visible = ((x - 2) * 2 + (y - 2) * 2) + 4;
    println!("{}", inner_visible + outer_visible);
}

fn observe_line_via_range_2<'a>(line: &mut impl Iterator<Item = &'a mut (i32, bool)>, initial_height: i32) {
    let mut highest = initial_height;
    for item in line{
        if item.0 > highest {
            highest = item.0;
            item.1 = true;
        }
    }
}

pub fn part2(){
    let lines = utils::read_file_to_array("/Users/eoghan/repos/aoc_rust_raw/src/8.txt", "\n")
        .unwrap()
        .iter()
        .map(|s| s.chars().map(|c| (c.to_digit(10).unwrap() as usize)).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let y_len = lines.len();
    let x_len = lines[0].len();

    let directions = [ViewedFrom::Bottom, ViewedFrom::Top, ViewedFrom::Left, ViewedFrom::Right].iter();
    
    let results = directions
        .map(move |direction| {
            
            let mut view_distances = lines
                .clone()
                .iter()
                .map(|l| l.iter().map(|_| 0).collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>();
            
            for coord in get_coordinate_iter(x_len, y_len, direction){
                let current_height = lines[coord.y][coord.x];
                let mut view_distance : usize = 0;
                {
                    let neighbor_iter = &mut get_neighbor_iter(coord.x, coord.y, &lines, direction);

                    while let Some(neightbor) = neighbor_iter.next(){
                        view_distance += 1;
                        if !is_higher(current_height, neightbor){
                            break;
                        }
                    }
                }
                view_distances[coord.y as usize][coord.x as usize] = view_distance;
            }
            view_distances
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    let result = get_coordinate_iter(x_len, y_len, &ViewedFrom::Bottom)
        .map(|coord| results.iter().map(|direction_distances| direction_distances[coord.y][coord.x]).fold(1, |a, b| a * b))
        .max()
        .unwrap();

    println!("{}", result);
}

enum ViewedFrom{
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Debug)]
struct Coordinate{
    x: usize,
    y: usize
}

fn get_coordinate_iter(width: usize, height: usize, direction: &ViewedFrom) -> Box<dyn Iterator<Item = Coordinate>>{
    match direction{
        ViewedFrom::Top => Box::new((0..width).flat_map(move |x| (0..height).map(move |y| Coordinate{x, y}))),
        ViewedFrom::Bottom => Box::new((0..width).flat_map(move |x| (0..height).rev().map(move |y| Coordinate{x, y}))),
        ViewedFrom::Left => Box::new((0..height).flat_map(move |y| (0..width).map(move |x| Coordinate{x, y}))),
        ViewedFrom::Right => Box::new((0..height).flat_map(move |y| (0..width).rev().map(move |x| Coordinate{x, y})))  
    }
}

fn get_neighbor_iter<'a>(x: usize, y: usize, grid: &'a Vec<Vec<usize>>, direction: &ViewedFrom) -> Box<dyn Iterator<Item =usize> + 'a> {
    match direction{
        ViewedFrom::Top => Box::new(grid
            .iter()
            .rev()
            .skip(grid.len() - y)
            .map(move |l| l[x])),
        ViewedFrom::Bottom => Box::new(grid
            .iter()
            .skip(y + 1)
            .map(move|l| l[x])),
        ViewedFrom::Left => Box::new(grid[y].iter().rev().skip(grid[y].len() - x).map(|z| *z)),
        ViewedFrom::Right => Box::new(grid[y].iter().skip(x + 1).map(|z| *z))
    }
}

fn is_higher(current_height: usize, neigbor_height : usize) -> bool{
    return current_height > neigbor_height;
}