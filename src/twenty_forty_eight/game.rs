use crate::twenty_forty_eight::structs::SIDE_SIZE;

use super::structs::{
    Board,
    Directions,
    MoveResult,
    Reason,
    Control
};

extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

const STDIN: i32 = 0;

pub fn next_keypress () -> u8 {

    let stdin = 0; 
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  
                                            
    new_termios.c_lflag &= !(ICANON | ECHO); 
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    let move_char = buffer[0];
    tcsetattr(stdin, TCSANOW, & termios).unwrap();  

    move_char
}


// w: 119
// a: 97
// s: 115
// d: 100

// u: 65
// l: 68
// d: 66
// r: 67

// k: 107
// h: 104
// j: 106
// l: 108

// q: 113

// e: 101

static HELP_STRING: &'static str = "Press: \n  (w, k, up arrow) to move up,\n  (a, h, right arrow) to move right,\n  (s, k, down arrow) to move down, (d, l, left arrow) to move left,\n  or 'q' to quit";

impl Control {
    fn process (self, board: Board, won: bool) -> MoveResult {
        match self {
            Control::Direction(dir) => {
                print!("{}[2J", 27 as char);

                // Get next board state
                let next_board = board.next_state(dir);
                println!("{}\nPress 'e' for help: ", next_board);

                // Check for a win
                if !won {
                    let Board(nb_tiles) = next_board;
                    if nb_tiles.iter().find(| elt | **elt == 2048).is_some() {
                        return MoveResult::Quit(Reason::Win(next_board));
                    }
                }

                // Check for a loss
                let Board(nb_tiles) = next_board;

                // First look for any empty tiles
                let empty = nb_tiles.iter().find(| elt | **elt == 0);
                if empty.is_none() {
                    // If there are no empty tiles, see if any neighbors are the same
                    let mut any_neighbors = false;
                    'out: for row in 0..SIDE_SIZE {
                        for col in 0..SIDE_SIZE {
                            let target = nb_tiles[ row * SIDE_SIZE + col ];

                            // up
                            if row != 0 {
                                let up = nb_tiles[ (row - 1) * SIDE_SIZE + col ];
                                if up == target {
                                    any_neighbors = true;
                                    break 'out;
                                }
                            }

                            // left
                            if col != 0 {
                                let left = nb_tiles[ row * SIDE_SIZE + col - 1 ];
                                if left == target {
                                    any_neighbors = true;
                                    break 'out;
                                }
                            }

                            // down
                            if row != SIDE_SIZE - 1 {
                                let up = nb_tiles[ (row + 1) * SIDE_SIZE + col ];
                                if up == target {
                                    any_neighbors = true;
                                    break 'out;
                                }
                            }

                            // right
                            if col != SIDE_SIZE - 1 {
                                let left = nb_tiles[ row * SIDE_SIZE + col + 1 ];
                                if left == target {
                                    any_neighbors = true;
                                    break 'out;
                                }
                            }
                        }
                    }

                    if !any_neighbors {
                        return MoveResult::Quit(Reason::Loss);
                    }
                }


                // Print new board

                MoveResult::Continue(next_board)
            },
            Control::Help => {

                // Print current board, and help string
                println!("{}\n{}", board, HELP_STRING);

                // And keep going
                MoveResult::Continue(board)
            },
            Control::Quit => MoveResult::Quit(Reason::QPressed),
        }
    }
}

pub fn next_move (board: Board, won: bool) -> MoveResult {

    
    loop {
        let keypress = {
            let keypress = next_keypress();
            if keypress == 27 && next_keypress() == 91 {
                next_keypress()
            }
            else {
                keypress
            }
        };

        let control = match keypress {
            // Up
            119 | 65 | 107 => Some(Control::Direction(Directions::Up)),
            // Left
            97 | 68 | 104 => Some(Control::Direction(Directions::Left)),
            // Down
            115 | 66 | 106 => Some(Control::Direction(Directions::Down)),
            // Right
            100 | 67 | 108 => Some(Control::Direction(Directions::Right)),
            // Quit
            113 => Some(Control::Quit),
            // Help
            101 => Some(Control::Help),
            _ => None
        };

        // While the the input key is invalid, continue reading keypresses
        if control.is_none() {
            continue;
        }

        return control.unwrap().process(board, won);
    }
}

use rand::Rng;

impl Board {
    pub fn new () -> Self {
        let mut tiles = [ 0; SIDE_SIZE * SIDE_SIZE ];
        let mut rng = rand::thread_rng();

        // Get the index to place the starter number
        let index = rng.gen_range(0..SIDE_SIZE*SIDE_SIZE);

        // Determine if the number should be a two or a four
        let is_four = rng.gen_bool(0.5);

        // Set the tile index to a 2 or a four
        tiles[index] = if is_four { 4 } else { 2 };

        // Return a new board
        Board(tiles)
    }

    fn next_state (self, direction: Directions) -> Board {

        fn shift_stack (
            tiles: &mut [u32; SIDE_SIZE * SIDE_SIZE], 
            row: usize, 
            col: usize, 
            do_iterate_rows: bool,
            do_iterate_backwards: bool,
        ) -> bool {

            let mut filled: [bool; SIDE_SIZE * SIDE_SIZE] = [false; SIDE_SIZE * SIDE_SIZE];

            let mut any_moved = false;

            // Closure that captures the "do_iterate_backwards" parameter, and moves a root
            //      usize index upwards or backwards by a provided offset
            let advance_by = | root: usize, by: usize| -> usize {
                if do_iterate_backwards {
                    root - by
                }
                else {
                    root + by
                }
            };

            let mut filled_row = row;
            let mut filled_col = col;

            for iter_off in 1..SIDE_SIZE {

                // Calculate the new row/col offsets for this iteration
                let (row, col) = if do_iterate_rows {
                    (advance_by(row, iter_off), col)
                }
                else {
                    (row, advance_by(col, iter_off))
                };
                let stack_val = tiles[row * SIDE_SIZE + col];


                // Don't fill anything in when the current number in the stack is 0
                if stack_val == 0 { continue }
                
                let filled_index = filled_row * SIDE_SIZE + filled_col;

                // If the current value is equal to the currently "filled" tile, then
                //      "merge" the tiles
                let do_erase = if stack_val == tiles[filled_index] && !filled[filled_index] {
                    // Merge by multiplying the tile value by 2
                    tiles[filled_index] = stack_val * 2;

                    // Then, denote that this slot has been filled once already
                    // This prevents someting like: [ 4, 4, 8, 0 ]
                    // From becoming: [ 16, 0, 0, 0 ]
                    // When the real desired state is: [ 8, 8, 0, 0 ]
                    filled[filled_index] = true;

                    true
                }
                
                // Otherwise, need to shift the tile down
                else if tiles[filled_index] == 0 {
                    // If the tile is shifting into an empty slot, then move the value over before advancing
                    //      filled index (row or column)
                    let do_erase = filled_row != row || filled_col != col;

                    // Simulate the "slide" by copying the value of the evauated tile into the 
                    //      same slot as the filled index
                    tiles[filled_index] = stack_val;
                    do_erase
                }
                else {

                    // If the tile being slid into is not empty, then advance the filled index
                    //      (row or column) before sliding
                    (filled_row, filled_col) = if do_iterate_rows {
                        (advance_by(filled_row, 1), filled_col)
                    }
                    else {
                        (filled_row, advance_by(filled_col, 1))
                    };

                    
                    let do_erase = filled_row != row || filled_col != col;

                    // Slide
                    tiles[filled_row * SIDE_SIZE + filled_col] = stack_val;

                    do_erase
                };

                if do_erase {
                    // And set the value of the "orginal tile" (the one that slide)
                    // To 0 -- a newly blank tile
                    let slide_from = row * SIDE_SIZE + col;
                    tiles[slide_from] = 0;

                    any_moved = true;
                }
            }

            any_moved
        }

        let Board(mut tiles) = self;

        let any_shifts = match direction {
            Directions::Up => {
                println!("Up");
                let mut any_shifted = false;
                // Iterate over the Cols
                for col in 0..SIDE_SIZE {
                    let row = 0;
                    let shifted = shift_stack(&mut tiles, row, col, true, false);
                    if shifted { any_shifted = true }
                };
                any_shifted
            },
            Directions::Down => {
                println!("Down");
                let mut any_shifted = false;
                // Iterate over the Cols
                for col in 0..SIDE_SIZE {
                    let row = SIDE_SIZE - 1;
                    let shifted = shift_stack(&mut tiles, row, col, true, true);
                    if shifted { any_shifted = true }
                };
                any_shifted
            },
            Directions::Left => {
                println!("Left");
                let mut any_shifted = false;
                // Iterate over the Rows
                for row in 0..SIDE_SIZE {
                    let col = 0;
                    let shifted = shift_stack(&mut tiles, row, col, false, false);
                    if shifted { any_shifted = true }
                };
                any_shifted
            },
            Directions::Right => {
                println!("Right");
                let mut any_shifted = false;
                // Iterate over the Rows
                for row in 0..SIDE_SIZE {
                    let col = SIDE_SIZE - 1;
                    let shifted = shift_stack(&mut tiles, row, col, false, true);
                    if shifted { any_shifted = true }
                };
                any_shifted
            }
        };

        // If none of the tiles moved, then return without adding a new tile
        if !any_shifts {
            return Board(tiles);
        }

        // If there was a shift, then add a new tile

        // Collect a list of the indeces of all empty tiles on the board
        let empty_tiles = tiles.iter()                                             // create an iterator
            .enumerate()                                                // add some indeces
            .filter(| (_index, elt) | **elt == 0)        // filter out any tile that has a non-zero value
            .map(| (index, _elt) | index)                       // map enumerated list to list of just the indexes
            .collect::<Vec<usize>>();                                                                   // collect vector of indeces

        let mut rng = rand::thread_rng();

        // Get the index of the empty_tiles vector in which to place the new tile
        let new_spot = rng.gen_range(0..empty_tiles.len());

        // Determine whether new value should be a 2 or a 4
        let is_four = rng.gen_bool(0.5);

        // Get the index of the new tile from the empty tiles array, and insert either a 2 or a 4 into that slot
        let new_tile_index = empty_tiles[new_spot];
        tiles[new_tile_index] = if is_four { 4 } else { 2 };

        // Return the next board state
        Board(tiles)
    }
}