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

    fn render(&self) {}

    pub fn move_up(&self) {}

    pub fn move_left(&self) {}

    pub fn move_down(&self) {}

    pub fn move_right(&self) {}

    pub fn update(&mut self) {
        self.render()
    }
}
