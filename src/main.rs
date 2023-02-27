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
        
        if check_win(&board) {
            break;
        }
        
        show_board(&board);
        
        let p2_column: usize = get_player_input(2, &column_heights);
        
        if p2_column == WIDTH + 1 {
            break;
        }
        
        place_piece(&mut board, &mut column_heights, 2, p2_column);
        
        if check_win(&board) {
            break;
        }
        
        show_board(&board);
    }
}

fn get_player_input(player: u8, column_heights: &[usize; WIDTH]) -> usize {
    loop {
        print!("Player {player}, pick a column (q to quit): ");
        io::stdout().flush();
        
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
            
            if column_heights[column] == HEIGHT {
                println!("That column is full");
                continue;
            }
            
            return column;
        }
    }
}

fn place_piece(board: &mut [char; WIDTH * HEIGHT], column_heights: &mut [usize; WIDTH], player: u8, column: usize) {
    board[get_board_index(HEIGHT - column_heights[column] - 1, column - 1)] = if player == 1 { 'x' } else { 'o' };
    column_heights[column] += 1;
}

// Find any horizontal 4-in-a-row sequences
// Returns x or o depending on player, or ' ' if none are found
fn get_horiz_sequence(board: &[char; WIDTH * HEIGHT]) -> char {
    let mut count: u8 = 0;
    for i in 0..HEIGHT {
        for j in 0..WIDTH-3 {
            let elem = board[get_board_index(i, j)];
            if elem == '-' { continue; }
            if elem == board[get_board_index(i, j + 1)] &&
               elem == board[get_board_index(i, j + 2)] &&
               elem == board[get_board_index(i, j + 3)] {
                   return elem;
               }
        }
    }
    ' '
}

// Find any vertical 4-in-a-row sequences
// Returns x or o depending on player, or ' ' if none are found
fn get_vert_sequence(board: &[char; WIDTH * HEIGHT]) -> char {
    let mut count: u8 = 0;
    for i in 0..HEIGHT-3 {
        for j in 0..WIDTH {
            let elem = board[get_board_index(i, j)];
            if elem == '-' { continue; }
            if elem == board[get_board_index(i + 1, j)] &&
               elem == board[get_board_index(i + 2, j)] &&
               elem == board[get_board_index(i + 3, j)] {
                   return elem;
               }
        }
    }
    ' '
}

// Find any positive diagonal 4-in-a-row sequences
// Returns x or o depending on player, or ' ' if none are found
fn get_diag_sequence(board: &[char; WIDTH * HEIGHT]) -> char {
    let mut count: u8 = 0;
    for i in 0..HEIGHT-3 {
        for j in 0..WIDTH-3 {
            let elem = board[get_board_index(i, j)];
            if elem == '-' { continue; }
            if elem == board[get_board_index(i + 1, j + 1)] &&
               elem == board[get_board_index(i + 2, j + 2)] &&
               elem == board[get_board_index(i + 3, j + 3)] {
                   return elem;
               }
        }
    }
    ' '
}

// Find any negative diagonal 4-in-a-row sequences
// Returns x or o depending on player, or ' ' if none are found
fn get_counter_diag_sequence(board: &[char; WIDTH * HEIGHT]) -> char {
    let mut count: u8 = 0;
    for i in 0..HEIGHT-3 {
        for j in 3..WIDTH {
            let elem = board[get_board_index(i, j)];
            if elem == '-' { continue; }
            if elem == board[get_board_index(i + 1, j - 1)] &&
               elem == board[get_board_index(i + 2, j - 2)] &&
               elem == board[get_board_index(i + 3, j - 3)] {
                   return elem;
               }
        }
    }
    ' '
}

fn find_sequence(board: &[char; WIDTH * HEIGHT]) -> char {
    let w = get_horiz_sequence(&board);
    if w != ' ' { return w; };
    
    let w = get_vert_sequence(&board);
    if w != ' ' { return w; };
    
    let w = get_diag_sequence(&board);
    if w != ' ' { return w; }
    
    let w = get_counter_diag_sequence(&board);
    if w != ' ' { return w; };
    
    ' '
}

fn check_win(board: &[char; WIDTH * HEIGHT]) -> bool {
    let w = find_sequence(&board);
    
    if w == ' ' { return false; }
    
    println!("");
    println!("Player {} wins!", if w == 'x' { 1 } else { 2 });
    println!("");
    show_board(&board);
    return true;
}

// Print the board to console
fn show_board(board: &[char; WIDTH * HEIGHT]) {
    println!("--- Board ---");
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{} ", board[get_board_index(i, j)]);
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

// Get board index from i, j 2d reference
fn get_board_index(i: usize, j: usize) -> usize {
    j * HEIGHT + i
}
