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

    pub fn is_valid_position(&self, pos: (i32, i32)) -> bool {
        let (y, x) = pos;
        y >= 0 && y < self.data.len() as i32 && x >= 0 && x < self.data[y as usize].len() as i32
    }

    pub fn is_wall(&self, pos: (i32, i32)) -> bool {
        let (y, x) = pos;
        if self.is_valid_position(pos) {
            let row = &self.data[y as usize];
            let ch = row.chars().nth(x as usize).unwrap();
            ch == self.wall_char
        } else {
            false
        }
    }

    pub fn get_map(&self) -> &Vec<String> {
        &self.data
    }

    pub fn get_wall_char(&self) -> char {
        self.wall_char
    }

    pub fn draw_map(&self) {
        for (y, line) in self.data.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let block_ch = if ch == self.wall_char {
                    '😀' as chtype
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
    let mut filled_area = Vec::new();
    let mut visited = vec![vec![false; level.data[0].len()]; level.data.len()];

    flood_fill_recursive(
        level,
        start,
        end,
        ignore_wall,
        &mut filled_area,
        &mut visited,
    );

    filled_area
}

fn flood_fill_recursive(
    level: &Level,
    pos: (i32, i32),
    end: (i32, i32),
    ignore_wall: char,
    filled_area: &mut Vec<(i32, i32)>,
    visited: &mut Vec<Vec<bool>>,
) {
    let (y, x) = pos;

    if !level.is_valid_position(pos) || visited[y as usize][x as usize] {
        return;
    }

    visited[y as usize][x as usize] = true;

    if level.is_wall(pos) && level.wall_char != ignore_wall {
        return;
    }

    filled_area.push(pos);

    if pos == end {
        return;
    }

    flood_fill_recursive(level, (y - 1, x), end, ignore_wall, filled_area, visited);
    flood_fill_recursive(level, (y + 1, x), end, ignore_wall, filled_area, visited);
    flood_fill_recursive(level, (y, x - 1), end, ignore_wall, filled_area, visited);
    flood_fill_recursive(level, (y, x + 1), end, ignore_wall, filled_area, visited);
}
