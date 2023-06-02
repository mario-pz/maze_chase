mod level;
use level::{flood_fill, Level};

mod entity;
use entity::Entity;

use ncurses::*;

use rand::Rng;

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

    let mut malph = Entity::new((16, 23), 'M', &level);
    let mut loukas = Entity::new((1, 1), 'L', &level);
    let mut diamond = Entity::new((10, 13), 'G', &level);

    let mut filled_area = flood_fill(
        &level,
        loukas.get_pos(),
        diamond.get_pos(),
        level.get_wall_char(),
    );

    let mut steps = 0;
    let mut rng = rand::thread_rng();
    let random_num: i32 = rng.gen_range(0..10);

    loop {
        clear();
        refresh();

        level.draw_map();
        loukas.update();
        malph.update();
        diamond.update();

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

        if random_num == steps {
            loop {
                let rand_x = rng.gen_range(0..20);
                let rand_y = rng.gen_range(0..20);
                if diamond.can_move((rand_y, rand_x)) {
                    diamond.move_to((rand_y, rand_x));
                    filled_area = flood_fill(
                        &level,
                        loukas.get_pos(),
                        diamond.get_pos(),
                        level.get_wall_char(),
                    );
                    break;
                }
            }
        }
        loukas.move_to(filled_area[steps as usize]);
        steps += 1;
    }

    // Cleanup ncurses
    endwin();
}
