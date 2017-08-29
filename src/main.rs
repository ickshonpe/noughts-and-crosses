extern crate rand;

use std::io;

#[derive(Clone, Copy, Debug)]
enum Piece {
    X,
    O
}

fn evaluate_square(square: &Option<Piece>) -> i64 {
    match *square {
        Some(Piece::X) => 1,
        Some(Piece::O) => -1,
        None => 0
    }
}

type Board = [[Option<Piece>; 3]; 3];

fn empty_board() -> Board {
    [[None; 3]; 3]
}

fn print_board(board: Board) {
    print!("   ");
    for x in 0..3 {
        print!(" {} ", x);
    }
    println!();
    for y in 0..3 {
        print!(" {} ", y);
        for x in 0..3 {
            let square = match board[x][y] {
                   Some(Piece::X) => "X",
                    Some(Piece::O) => "O",
                    _ => "."
                };
            print!(" {} ", square);
        }
        println!();        
    }
}

fn read_player_input() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        if let Ok(num) = input.trim().parse::<usize>() {
            if num < 3 {
                return num
            }
        } 
        println!("Please enter a number between 0 and 2");        
    } 
}

fn evaluate_line(line: [Option<Piece>; 3]) -> Option<Piece> {
    let sum = line.iter().map(evaluate_square).sum();
    match sum {
        3 => Some(Piece::X),
        -3 => Some(Piece::O),
        _ => None
    }
}

fn check_columns(board: &Board) -> Option<Piece> {
    for x in 0 .. 3 {
        let column = board[x];
        let result = evaluate_line(column);
        if result.is_some() {
            return result
        }
    }
    None
}

fn check_rows(board: &Board) -> Option<Piece> {
    for y in 0 .. 3 {
        let row = [board[0][y], board[1][y], board[2][y]];
        let result = evaluate_line(row);
        if result.is_some() {
            return result
        }
    }
    None
}

fn check_diagonals(board: &Board) -> Option<Piece> {
    let d = [board[0][0], board[1][1], board[2][2]];
    let result = evaluate_line(d);
    if result.is_some() {
        return result
    }
    let d = [board[0][2], board[1][1], board[2][0]];
    evaluate_line(d)    
}

fn evaluate_board(board: &Board) -> Option<Piece> {
    let cs = check_columns(board);
    let rs = check_rows(board);
    let ds = check_diagonals(board);
    for result in [cs, ds, rs].iter() {
        if result.is_some() {
            return *result
        }
    }
    None
}

// advanced machine intelligence bit here
fn make_computer_move(board: &mut Board) {
    use rand::Rng;
    loop {
        let x: usize = rand::thread_rng().gen_range(0, 3);
        let y: usize = rand::thread_rng().gen_range(0, 3);
        if board[x][y].is_none() {
            board[x][y] = Some(Piece::O);
            break;
        }
    }
}

fn main() {
    let mut board: Board = empty_board();    
    let mut moves = 0;
    loop {
        'input: loop {
            print_board(board);
            println!("column?");
            let x = read_player_input();
            println!("row?");
            let y = read_player_input();
            if board[x][y].is_none() {
                board[x][y] = Some(Piece::X);
                break 'input;
            } else {
                println!("Invalid move, choose an unoccupied square!");
            }
        }        
        moves += 1;
        let result = evaluate_board(&board);
        if let Some(winner) = result {
            print_board(board);
            let winner = match winner { Piece::X => "Xs", Piece::O => "Os" };
            println!("{} win!", winner);
            std::process::exit(0)
        }
        if moves == 9 {
            print_board(board);
            println!("You DRAW!");
            std::process::exit(0)
        }
        make_computer_move(&mut board);
        moves += 1;
    }
}
