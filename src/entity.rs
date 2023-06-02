use ncurses::*;

use crate::level::Level;

pub struct Entity<'a> {
    position: (i32, i32),
    character: char,
    level: &'a Level,
}

impl<'a> Entity<'a> {
    pub fn new(position: (i32, i32), character: char, level: &Level) -> Entity {
        Entity {
            position,
            character,
            level,
        }
    }

    pub fn can_move(&self, position: (i32, i32)) -> bool {
        let data = self.level;
        let map = data.get_map();
        let wall = data.get_wall_char();

        let y = position.0 as usize;
        let x = position.1 as usize;

        if y >= map.len() || x >= map[y].len() {
            return false;
        }

        let ch = map[y].chars().nth(x);

        if let Some(cell) = ch {
            if cell == wall {
                return false;
            }
        }

        true
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.position
    }

    fn render(&self) {
        mvaddch(self.position.1, self.position.0, self.character as chtype);
    }

    pub fn move_up(&mut self) {
        let new_position = (self.position.0, self.position.1 - 1);
        if self.can_move(new_position) {
            self.move_to(new_position);
        }
    }

    pub fn move_left(&mut self) {
        let new_position = (self.position.0 - 1, self.position.1);
        if self.can_move(new_position) {
            self.move_to(new_position);
        }
    }

    pub fn move_down(&mut self) {
        let new_position = (self.position.0, self.position.1 + 1);
        if self.can_move(new_position) {
            self.move_to(new_position);
        }
    }

    pub fn move_right(&mut self) {
        let new_position = (self.position.0 + 1, self.position.1);
        if self.can_move(new_position) {
            self.move_to(new_position);
        }
    }

    pub fn move_to(&mut self, position: (i32, i32)) {
        self.position = position;
    }

    pub fn update(&mut self) {
        self.render()
    }
}
