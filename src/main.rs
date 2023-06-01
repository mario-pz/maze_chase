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

    let level = Level::new("assets/map1.txt").unwrap_or_else(|_| {
        println!("Failed to load level.");
        std::process::exit(1);
    });

    let data = level.get_data();
    println!("{:?}", data);

    let mut loukas = Entity::new((1, 1), 'L');
    let mut malph = Entity::new((4, 1), 'M');

    loop {
        clear();
        refresh();

        let key = getch();
        match key {
            KEY_UP => loukas.move_up(),
            KEY_DOWN => loukas.move_down(),
            KEY_LEFT => loukas.move_left(),
            KEY_RIGHT => loukas.move_right(),
            27 => {
                break;
            }
            _ => (),
        }

        loukas.update();
        malph.update();
    }

    // Cleanup ncurses
    endwin();
}
