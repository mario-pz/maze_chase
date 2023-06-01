use ncurses::*;

#[allow(dead_code)]
pub struct Entity {
    position: (i32, i32),
    character: char,
}

impl Entity {
    pub fn new(position: (i32, i32), character: char) -> Entity {
        Entity {
            position,
            character,
        }
    }

    #[allow(dead_code)]
    pub fn get_pos(&self) -> &(i32, i32) {
        &self.position
    }

    #[allow(dead_code)]
    pub fn get_char(&self) -> &char {
        &self.character
    }

    fn render(&self) {
        mvaddch(self.position.1, self.position.0, self.character as chtype);
    }

    pub fn move_up(&mut self) {
        self.position.1 -= 1;
    }

    pub fn move_left(&mut self) {
        self.position.0 -= 1;
    }

    pub fn move_down(&mut self) {
        self.position.1 += 1;
    }

    pub fn move_right(&mut self) {
        self.position.0 += 1;
    }

    pub fn update(&mut self) {
        self.render()
    }
}
