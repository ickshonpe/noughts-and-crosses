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
    println!("-=--=--=--=-");
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
    println!("-=--=--=--=-");
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

fn evaluate_board(board: &Board) -> Option<Piece> {
    for result in
        (0 .. 3).map(|x| board[x])
        .chain((0 .. 3).map(|y| [board[0][y], board[1][y], board[2][y]]))
        .chain(iter::once([board[0][0], board[1][1], board[2][2]]))
        .chain(iter::once([board[0][2], board[1][1], board[2][0]]))
        .map(evaluate_line) {
        if result.is_some() {
            return result
        }
    }
    None
}

// advanced machine intelligence bit here
fn make_computers_move(board: &mut Board) {    
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
    println!("Noughts VS Crosses");
    println!();
    let mut board: Board = empty_board();    
    let mut moves = 0;
    loop {
        'input: loop {
            print_board(board);
            println!("enter column:");
            let x = read_player_input();
            println!("enter row:`");
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
            let winner = match winner { Piece::X => "You win!", Piece::O => "Computer wins!" };
            println!("{}", winner);
            std::process::exit(0)
        }
        if moves == 9 {
            print_board(board);
            println!("You DRAW!");
            std::process::exit(0)
        }
        make_computers_move(&mut board);
        moves += 1;
    }
}
