use std::fmt;
use std::fmt::write;
use super::structs::Directions;
use super::structs::Board;
use super::structs::SIDE_SIZE;

pub fn print() {
    let dir = Directions::Up;
    println!("Directions: {:?}", dir);
    println!("Hello");
}

impl Board {
    
}

fn find_max_tilesize (board: &Board) -> (u8, Vec<(u8, u32)>) {
    let Board(tiles) = board;

    // Assign a tile size for each of the tiles in the board
    let size_values = tiles.iter().map(| item | {
        let mut digits = 1;
        let mut element = *item;
        while element / 10 > 0 {
            digits += 1;
            element /= 10;
        };

        // Return a tuple of digits and the tile item itself
        (digits, *item)

    // Collect into a vector of sizes and tile values
    }).collect::<Vec<(u8, u32)>>();


    // Max size
    let max = size_values.iter().max_by_key(| (digits, _) | digits).unwrap().0;

    (max, size_values)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let (max_size, sizes_and_values) = find_max_tilesize(self);

        let bar_size = 
            (
                max_size              // size of largest-digited number on the board, 
                + 2                   // plus space padding
                + 1                   // plus pipe padding
            ) * SIDE_SIZE as u8       // All multiplied times the amount of tiles within each side
            + 1;                      // plus the off-by-one-error-fixer

        // Render the top bar
        for _ in 0..bar_size {
            let _ = write!(f, "-");
        }
        let _ = writeln!(f, "");

        sizes_and_values.iter().enumerate().for_each(| (index, (size, tile)) | {

            // Difference between the size of the largest-digited tile, and this tile
            // Used for determining how many spaces to pad this number with
            let space_diff = max_size - size;

            // Get spacing for either side
            let right_spacing = space_diff / 2;                 // integer division rounds down
            let left_spacing = space_diff - right_spacing;      // rounded number space goes to left
            
            // Add one extra r/l spacing for prettiness
            let right_spacing = right_spacing + 1;
            let left_spacing = left_spacing + 1;

            // Tile divider
            let _ = write!(f, "|");

            // Render the left spacing to the console
            for _ in 0..left_spacing {
                let _ = write!(f, " ");
            }

            // Render the tile number value, as long as the tile values is not 0 
            if *tile != 0 {
                let _ = write!(f, "{}", *tile);
            }
            else {
                // 0 tile value signifies "blank", and should not be rendered
                // But should take up space
                // Render a blank instead
                let _ = write!(f, " ");
            }

            // Render the right spacing
            for _ in 0..right_spacing {
                let _ = write!(f, " ");
            }

            // Every SIDE_SIZE tile, render a pipe character and a newline
            if (index + 1) % SIDE_SIZE == 0 {
                let _ = writeln!(f, "|");
            }
        });

        // Finally render the bottom bar
        for _ in 0..bar_size {
            let _ = write!(f, "-");
        }
        writeln!(f, "")
    }
}