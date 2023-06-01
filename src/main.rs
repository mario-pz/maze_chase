mod level;
use level::Level;

mod entity;
use entity::Entity;

use ncurses::*;

fn main() {
    // Initialize ncurses
    initscr();
    keypad(stdscr(), true); // Enable keypad for special keys
    noecho(); // Disable automatic echoing of typed characters

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let level = Level::new("levels/map1.txt").unwrap_or_else(|_| {
        println!("Failed to load level.");
        std::process::exit(1);
    });

    let mut loukas = Entity::new((1, 1), 'L');
    let mut malph = Entity::new((4, 1), 'M');

    loop {
        clear();
        refresh();

        level.draw_map();
        loukas.update();
        malph.update();

        let key = getch();
        match key {
            KEY_UP | 107 => loukas.move_up(),       // ASCII value for 'k'
            KEY_DOWN | 106 => loukas.move_down(),   // ASCII value for 'j'
            KEY_LEFT | 104 => loukas.move_left(),   // ASCII value for 'h'
            KEY_RIGHT | 108 => loukas.move_right(), // ASCII value for 'l'
            27 => {
                endwin();
                break;
            }
            _ => (),
        }
    }

    // Cleanup ncurses
    endwin();
}
