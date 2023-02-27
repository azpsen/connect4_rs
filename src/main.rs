use std::io;
use std::io::Write;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

fn main() {
    println!("");
    println!("---- Welcome to Connect 4! ----");
    println!("");

    let mut board: [char; WIDTH * HEIGHT] = ['-'; WIDTH * HEIGHT];
    let mut column_heights: [usize; WIDTH] = [0; WIDTH];
    
    show_board(&board);
    
    loop {
        let p1_column: usize = get_player_input(1, &column_heights);
        
        if p1_column == WIDTH + 1 {
            break;
        }
        
        place_piece(&mut board, &mut column_heights, 1, p1_column);
        
        show_board(&board);
        
        let p2_column: usize = get_player_input(2, &column_heights);
        
        if p2_column == WIDTH + 1 {
            break;
        }
        
        place_piece(&mut board, &mut column_heights, 2, p2_column);
        
        show_board(&board);
    }
}

fn get_player_input(player: u8, column_heights: &[usize; WIDTH]) -> usize {
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
            
            if (column_heights[column] == HEIGHT) {
                println!("That column is full");
                continue;
            }
            
            return column;
        }
    }
}

fn place_piece(board: &mut [char; WIDTH * HEIGHT], column_heights: &mut [usize; WIDTH], player: u8, column: usize) {
    board[(column - 1) * HEIGHT + (HEIGHT - column_heights[column] - 1)] = if player == 1 { 'x' } else { 'o' };
    column_heights[column] += 1;
}

// Print the board to console
fn show_board(board: &[char; WIDTH * HEIGHT]) {
    println!("--- Board ---");
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{} ", board[j * HEIGHT + i]);
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
