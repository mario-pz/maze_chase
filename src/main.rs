mod level;
use level::{flood_fill, Level};

mod entity;
use entity::Entity;

use ncurses::*;

fn main() {
    // Initialize ncurses
    initscr();
    keypad(stdscr(), true); // Enable keypad for special keys
    noecho(); // Disable automatic echoing of typed characters

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let level = Level::new("levels/map1.txt", '*').unwrap_or_else(|_| {
        println!("Failed to load level.");
        std::process::exit(1);
    });

    let mut loukas = Entity::new((1, 1), 'L', &level);
    let mut malph = Entity::new((4, 1), 'M', &level);

    let mut filled_area = flood_fill(
        &level,
        loukas.get_pos(),
        malph.get_pos(),
        level.get_wall_char(),
    );

    let mut steps = 0;

    loop {
        clear();
        refresh();

        level.draw_map();
        loukas.update();
        malph.update();

        if steps % 6 == 0 {
            filled_area = flood_fill(&level, loukas.get_pos(), malph.get_pos(), '*');
        }

        let key = getch();
        match key {
            KEY_UP | 107 => malph.move_up(),       // ASCII value for 'k'
            KEY_DOWN | 106 => malph.move_down(),   // ASCII value for 'j'
            KEY_LEFT | 104 => malph.move_left(),   // ASCII value for 'h'
            KEY_RIGHT | 108 => malph.move_right(), // ASCII value for 'l'
            27 => {
                endwin();
                break;
            }
            _ => (),
        }

        let index = steps % filled_area.len(); // Calculate the wrapped index
        loukas.move_to(filled_area[index]);
        steps = steps.wrapping_add(1); // Use wrapping_add to prevent overflow
    }

    // Cleanup ncurses
    endwin();
}
