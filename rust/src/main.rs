use std::{thread, time};
use rand::Rng;

// Tree parameters
const TREE_HEIGHT: usize = 30;
const ROWS_BEFORE_BREAK: usize = TREE_HEIGHT / 3;
const STUMP_HEIGHT: usize = 5;
const STUMP_WIDTH: usize = 5;
const TREE_WIDTH: usize = (TREE_HEIGHT * 2) - 1; // Calculate correct tree width based on height
const REQUIRED_SNOW_PARTICLES: usize = 10;
const SLEEP_TIME_MICROSECONDS: u64 = 125_000;

fn build_picture(picture: &mut Vec<Vec<char>>) {
    picture.clear(); // Clear any existing picture

    // Build the tree
    let mut current_width: usize = 1;
    for y in 0..TREE_HEIGHT {
        if y % ROWS_BEFORE_BREAK == 0 {
            current_width = current_width.saturating_sub(8); // Prevent negative width
        }

        let mut row = vec![' '; TREE_WIDTH];
        let space: usize = (TREE_WIDTH - current_width) / 2;
        for x in space..(space + current_width) {
            row[x] = '*';
        }
        picture.push(row);
        current_width += 2; // Increase the width of the tree gradually
    }

    // Build the stump
    let space: usize = (TREE_WIDTH - STUMP_WIDTH) / 2;
    for _y in 0..STUMP_HEIGHT {
        let mut row = vec![' '; TREE_WIDTH];
        for x in space..(space + STUMP_WIDTH) {
            row[x] = '#';
        }
        picture.push(row);
    }

    // Message
    let message = "Merry Christmas!";
    let message_space: usize = (TREE_WIDTH - message.chars().count()) / 2;
    let mut char_counter: usize = 0;

    for y in 1..=3 {
        let mut row = vec![' '; TREE_WIDTH];
        if y == 1 || y == 3 {
            picture.push(row);
        } else {
            for x in message_space..(message_space + message.chars().count()) {
                if let Some(c) = message.chars().nth(char_counter) {
                    row[x] = c;
                    char_counter += 1;
                }
            }
            picture.push(row);
        }
    }
}

fn draw_picture(picture: &Vec<Vec<char>>) {
    for row in picture {
        for ch in row {
            print!("{}", ch);
        }
        println!(); // Print a new line after each row
    }
}

fn add_snow(picture: &mut Vec<Vec<char>>) {
    let mut rng = rand::thread_rng();
    let mut added_snow_particles = 0;

    while added_snow_particles < REQUIRED_SNOW_PARTICLES {
        let rand_y = rng.gen_range(0..(TREE_HEIGHT + STUMP_HEIGHT)); // Random Y coordinate within the tree height
        let rand_x = rng.gen_range(0..TREE_WIDTH); // Random X coordinate within the tree width

        // Add snow only to empty spaces
        if picture[rand_y][rand_x] == ' ' {
            picture[rand_y][rand_x] = 'x';
            added_snow_particles += 1;
        }
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn main() {
    let mut picture: Vec<Vec<char>> = vec![vec![' '; TREE_WIDTH]; TREE_HEIGHT + STUMP_HEIGHT + 3]; // Allocate space for the picture

    loop {
        // Clear the console (works for most terminal emulators)
        clear_screen();
        
        build_picture(&mut picture);
        add_snow(&mut picture);
        draw_picture(&picture);

        // Pause for the given time (converted from microseconds to milliseconds)
        let sleep_duration = time::Duration::from_micros(SLEEP_TIME_MICROSECONDS);
        thread::sleep(sleep_duration);
    }
}

