use crate::twenty_forty_eight::structs::MoveResult;

use std::process;

mod twenty_forty_eight;


fn main() {
    print!("{}[2J", 27 as char);
    let mut board = twenty_forty_eight::structs::Board::new();
    
    println!("{}", board);
    println!("Press 'e' for help");

    let mut won = false;

    loop {
        let move_res = twenty_forty_eight::game::next_move(board, won);
        board = match move_res {
            MoveResult::Continue( new_state ) => {
                new_state
            },
            MoveResult::Quit( reason ) => {
                match reason {
                    twenty_forty_eight::structs::Reason::QPressed => {
                        // Stop playing when q is pressed
                        // Do this by setting cont to false
                        process::exit(0x0);
                    },
                    twenty_forty_eight::structs::Reason::Win( board ) => {
                        println!("You win!");
                        println!("Play again?\n  'y' to start new game\n  'c' to continue playing this game\n  'q' to quit");
                        
                        let get_resp = || {
                            
                            loop {
                                // Prompt
                                println!(": ");
    
                                // Get response from user
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input);
    
                                // Get the first character from the user input
                                let char_resp = if let Some(cr) = input.chars().nth(0) { cr }
                                // If the user gave an empty string, simply continue looping
                                else {
                                    continue
                                };
    
                                match char_resp {
                                    // 'y' -- new game
                                    'y' | 'Y' => {
                                        won = false;
                                        return Some(twenty_forty_eight::structs::Board::new());
                                    },
                                    // 'c' -- continue
                                    'c' | 'C' => {
                                        // Continue playing with the current board
                                        won = true;
                                        return Some(board);
                                    },
                                    // 'q' -- quit
                                    'q' | 'Q'  => {
                                        // Set continue to false, and break
                                        return None;
                                    },
                                    _ => {}
                                }
                            }
                        };


                        let resp  = get_resp();
                        if let Some(board) = resp {
                            print!("{}[2J", 27 as char);
                            println!("{}", board);
                            println!("Press 'e' for help:");
                            board
                        }
                        else {
                            process::exit(0x0)
                        }
                    },
                    twenty_forty_eight::structs::Reason::Loss => {
                        println!("You lose! Loser!");
                        println!("Play again?\n  'y' to start new game\n  'q' to quit");

                        let get_resp = || {
                            
                            loop {
                                // Prompt
                                println!(": ");
    
                                // Get response from user
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input);
    
                                // Get the first character from the user input
                                let char_resp = if let Some(cr) = input.chars().nth(0) { cr }
                                // If the user gave an empty string, simply continue looping
                                else {
                                    continue
                                };
    
                                match char_resp {
                                    // 'y' -- new game
                                    'y' | 'Y' => {
                                        return Some(twenty_forty_eight::structs::Board::new());
                                    },
                                    // 'q' -- quit
                                    'q' | 'Q'  => {
                                        // Set continue to false, and break
                                        return None;
                                    },
                                    _ => {}
                                }
                            }
                        };

                        let resp = get_resp();
                        if let Some(board) = resp {
                            print!("{}[2J", 27 as char);
                            println!("{}", board);
                            println!("Press 'e' for help:");
                            board
                        }
                        else {
                            process::exit(0x0);
                        }
                    },
                }
            },
            MoveResult::Err(err) => panic!("Error: {}", err)
        };
    }


    // println!("{}", board);
}
