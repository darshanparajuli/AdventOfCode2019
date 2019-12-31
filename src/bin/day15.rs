use aoc_2019::intcode_computer::*;
use aoc_2019::read_input;
use std::collections::{HashSet, VecDeque};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?
        .iter()
        .map(|s| s.split(",").map(|a| a.parse::<i64>()))
        .flatten()
        .collect::<Result<Vec<_>, _>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &[i64]) {
    let c = IntCodeComputer::new(input.to_vec());
    let mut visited = HashSet::new();
    let mut walls = HashSet::new();
    let mut grid = Grid::new(50, 50);

    explore(c, Point::new(0, 0), 1, &mut grid, &mut visited, &mut walls);

    let mut visited = HashSet::new();
    let mut result = 0;

    grid_dfs(Point::new(0, 0), &grid, &mut visited, 0, &mut result);

    println!("part 1: {}", result);
}

fn part2(input: &[i64]) {
    let c = IntCodeComputer::new(input.to_vec());
    let mut visited = HashSet::new();
    let mut walls = HashSet::new();
    let mut grid = Grid::new(50, 50);

    explore(c, Point::new(0, 0), 1, &mut grid, &mut visited, &mut walls);

    let oxygen = {
        let mut p = None;
        for x in -25..25 {
            for y in -25..25 {
                if grid.get(x, y) == Cell::Oxygen {
                    p = Some(Point::new(x, y));
                }
            }
        }

        p.unwrap()
    };

    let mut queue = VecDeque::new();
    queue.push_back(oxygen);

    let mut minutes = 0;
    let mut next = vec![];

    loop {
        next.clear();
        while let Some(p) = queue.pop_front() {
            for i in 1..=4 {
                let tmp = p.move_to_dir(i);
                if grid.get(tmp.x, tmp.y) == Cell::Valid {
                    next.push(tmp);
                    grid.set(tmp.x, tmp.y, Cell::Oxygen);
                }
            }
        }

        if next.is_empty() {
            break;
        } else {
            for i in &next {
                queue.push_back(*i);
            }
            minutes += 1;
        }
    }

    println!("part 2: {}", minutes);
}

fn grid_dfs(p: Point, grid: &Grid, visited: &mut HashSet<Point>, acc: u32, result: &mut u32) {
    if grid.get(p.x, p.y) == Cell::Wall {
        return;
    }

    if grid.get(p.x, p.y) == Cell::Oxygen {
        *result = acc;
        return;
    }

    visited.insert(p);
    for i in 1..=4 {
        let pp = p.move_to_dir(i);
        if !visited.contains(&pp) {
            grid_dfs(pp, grid, visited, acc + 1, result);
        }
    }
}

fn explore(
    mut computer: IntCodeComputer,
    p: Point,
    dir: u8,
    grid: &mut Grid,
    visited: &mut HashSet<Point>,
    walls: &mut HashSet<Point>,
) {
    visited.insert(p);
    if grid.get(p.x, p.y) == Cell::Empty {
        grid.set(p.x, p.y, Cell::Valid);
    }

    match computer.run() {
        Ret::In(input) => input.write(dir as i64),
        _ => unreachable!("Error writing input"),
    }

    let output = match computer.run() {
        Ret::Out(output) => output,
        _ => unreachable!("Error getting output"),
    };

    match output {
        0 => {
            let w = p.move_to_dir(dir);
            grid.set(w.x, w.y, Cell::Wall);
            walls.insert(w);

            for i in (1..=4).filter(|d| *d != dir) {
                let new = p.move_to_dir(i);
                if !walls.contains(&new) && !visited.contains(&new) {
                    explore(computer.clone(), p, i, grid, visited, walls);
                }
            }
        }
        1 => {
            explore(
                computer.clone(),
                p.move_to_dir(dir),
                dir,
                grid,
                visited,
                walls,
            );

            match computer.run() {
                Ret::In(input) => input.write(opposite_dir(dir) as i64),
                _ => unreachable!("Error writing input"),
            }

            let _ = computer.run();

            for i in (1..=4).filter(|d| *d != dir) {
                let new = p.move_to_dir(i);
                if !walls.contains(&new) && !visited.contains(&new) {
                    explore(computer.clone(), p, i, grid, visited, walls);
                }
            }
        }
        2 => {
            let o = p.move_to_dir(dir);
            grid.set(o.x, o.y, Cell::Oxygen);

            explore(
                computer.clone(),
                p.move_to_dir(dir),
                dir,
                grid,
                visited,
                walls,
            );

            match computer.run() {
                Ret::In(input) => input.write(opposite_dir(dir) as i64),
                _ => unreachable!("Error writing input"),
            }

            let _ = computer.run();

            for i in (1..=4).filter(|d| *d != dir) {
                let new = p.move_to_dir(i);
                if !walls.contains(&new) && !visited.contains(&new) {
                    explore(computer.clone(), p, i, grid, visited, walls);
                }
            }
        }
        _ => unreachable!(),
    }
}

fn opposite_dir(dir: u8) -> u8 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => unreachable!(),
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_by(&self, dx: i32, dy: i32) -> Self {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn move_to_dir(&self, dir: u8) -> Self {
        // North = 1
        // South = 2
        // West = 3
        // East = 4
        match dir {
            1 => self.move_by(0, 1),
            2 => self.move_by(0, -1),
            3 => self.move_by(-1, 0),
            4 => self.move_by(1, 0),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Wall,
    Valid,
    Empty,
    Oxygen,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Grid {
    array: Vec<Vec<Cell>>,
    offx: u32,
    offy: u32,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            array: vec![vec![Cell::Empty; cols]; rows],
            offx: cols as u32 / 2,
            offy: rows as u32 / 2,
            rows,
            cols,
        }
    }

    fn with_offset(&self, x: i32, y: i32) -> (usize, usize) {
        let x = self.offx as i32 + x;
        let y = self.offy as i32 + y;
        if x < 0 || x >= self.cols as i32 {
            panic!("x = {}", x);
        }
        if y < 0 || y >= self.rows as i32 {
            panic!("y = {}", y);
        }
        (x as usize, y as usize)
    }

    fn set(&mut self, x: i32, y: i32, cell: Cell) {
        let (x, y) = self.with_offset(x, y);
        self.array[y][x] = cell;
    }

    fn get(&self, x: i32, y: i32) -> Cell {
        let (x, y) = self.with_offset(x, y);
        self.array[y][x]
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                if x == self.offx as usize && y == self.offy as usize {
                    print!("X ");
                } else {
                    match self.array[y][x] {
                        Cell::Wall => print!("# "),
                        Cell::Valid => print!(". "),
                        Cell::Empty => print!("  "),
                        Cell::Oxygen => print!("O "),
                    }
                }
            }
            println!();
        }
        println!();
    }
}
