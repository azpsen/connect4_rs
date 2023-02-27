use std::io;
use std::io::Write;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

const BOARD: [char; WIDTH * HEIGHT] = ['o'; WIDTH * HEIGHT];

fn main() {
    println!("");
    println!("---- Welcome to Connect 4! ----");
    println!("");
    
    let mut p1_column = String::new();
    let mut p2_column = String::new();
    
    'game: loop {
        show_board();
        
        'p1_guess: loop {
            println!("Player 1, pick a column (q to quit): ");
            
            let mut p1_column = String::new();
            io::stdin()
                .read_line(&mut p1_column)
                .expect("Failed to read input");
            
            if p1_column.trim() == "q" {
                break 'game;
            } else {
                let p1_column : usize = match p1_column.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a number from 1 to {}", WIDTH);
                        continue 'p1_guess;
                    }
                };
                if p1_column > WIDTH {
                    println!("Please enter a number from 1 to {}", WIDTH);
                    continue 'p1_guess;
                }
            }
        }
        
        
    }
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
