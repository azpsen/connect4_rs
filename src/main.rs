use std::io;
use std::io::Write;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

const BOARD: [char; WIDTH * HEIGHT] = ['-'; WIDTH * HEIGHT];

fn main() {
    println!("");
    println!("---- Welcome to Connect 4! ----");
    println!("");
    
    show_board();
    
    loop {
        let p1_column: usize = get_player_input(1);
        
        if p1_column == WIDTH + 1 {
            break;
        }
        
        place_piece(1, p1_column);
        
        show_board();
        
        let p2_column: usize = get_player_input(2);
        
        if p2_column == WIDTH + 1 {
            break;
        }
        
        place_piece(2, p2_column);
        
        show_board();
    }
}

fn get_player_input(player: u8) -> usize {
    loop {
        println!("Player {player}, pick a column (q to quit): ");
        
        let mut column = String::new();
        io::stdin()
            .read_line(&mut column)
            .expect("Failed to read input");
        
        if column.trim() == "q" {
            return WIDTH + 1;
        } else {
            let column: usize = match column.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number from 1 to {}", WIDTH);
                    continue;
                }
            };
            if column > WIDTH {
                println!("Please enter a number from 1 to {}", WIDTH);
                continue;
            }
            
            return column;
        }
    }
}

fn place_piece(player: u8, column: usize) {
    
}

// Print the board to console
fn show_board() {
    println!("--- Board ---");
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{} ", BOARD[j * HEIGHT + i]);
            io::stdout().flush().unwrap();
        }
        println!("");
    }
    
    println!("-------------");
    
    // Print the column numbers
    for i in 0..WIDTH {
        print!("{} ", i + 1);
        io::stdout().flush().unwrap();
    }
    println!("");
    println!("^^ Columns ^^");
    println!("");
}
