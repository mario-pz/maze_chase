use ncurses::*;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub struct Level {
    data: Vec<String>,
    wall_char: char,
}

impl Level {
    pub fn new(file_path: &str, wall_char: char) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut data = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                data.push(line);
            }
        }

        Ok(Level { data, wall_char })
    }

    pub fn get_map(&self) -> &Vec<String> {
        &self.data
    }

    pub fn get_wall_char(&self) -> char {
        self.wall_char
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }

    pub fn draw_map(&self) {
        for (y, line) in self.data.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let block_ch = if ch == self.wall_char {
                    'X' as chtype
                } else {
                    ch as chtype
                };

                mvaddch(y as i32, x as i32, block_ch);
            }
        }
    }
}

pub fn flood_fill(
    level: &Level,
    start: (i32, i32),
    end: (i32, i32),
    ignore_wall: char,
) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    let mut visited: Vec<Vec<bool>> =
        vec![vec![false; level.get_map_size().1]; level.get_map_size().0];

    fill(
        level,
        start.0,
        start.1,
        end.0,
        end.1,
        ignore_wall,
        &mut result,
        &mut visited,
    );

    result
}

fn fill(
    level: &Level,
    x: i32,
    y: i32,
    end_x: i32,
    end_y: i32,
    ignore_wall: char,
    result: &mut Vec<(i32, i32)>,
    visited: &mut Vec<Vec<bool>>,
) {
    if x < 0 || y < 0 || x >= level.get_map_size().0 as i32 || y >= level.get_map_size().1 as i32 {
        return;
    }

    if visited[x as usize][y as usize] {
        return;
    }

    visited[x as usize][y as usize] = true;

    if level.data[x as usize].chars().nth(y as usize) != Some(ignore_wall) {
        result.push((x, y));
    }

    if x == end_x && y == end_y {
        return;
    }

    // Check neighbors (up, down, left, right)
    fill(level, x - 1, y, end_x, end_y, ignore_wall, result, visited);
    fill(level, x + 1, y, end_x, end_y, ignore_wall, result, visited);
    fill(level, x, y - 1, end_x, end_y, ignore_wall, result, visited);
    fill(level, x, y + 1, end_x, end_y, ignore_wall, result, visited);
}
