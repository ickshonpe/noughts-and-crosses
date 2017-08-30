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
    print!("-=--=--=--=-\n   ");
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
                return num;
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
        (0..3).map(|x| board[x])
            .chain((0..3).map(|y| [board[0][y], board[1][y], board[2][y]]))
            .chain(vec![[board[0][0], board[1][1], board[2][2]], [board[0][2], board[1][1], board[2][0]]])
            .map(evaluate_line) {
        if result.is_some() {
            return result;
        }
    }
    None
}


// advanced machine intelligence bit here
fn make_computers_move(board: &mut Board) {
    use rand::Rng;
    loop {
        let x = rand::thread_rng().gen_range(0, 3);
        let y = rand::thread_rng().gen_range(0, 3);
        if board[x][y].is_none() {
            board[x][y] = Some(Piece::O);
            break;
        }
    }
}

fn main() {
    println!("Noughts & Crosses\n");
    let mut board: Board = empty_board();
    let mut turn = 0;
    while turn < 9 {
        if turn % 2 == 0 {
            'input: loop {
                print_board(board);
                println!("enter column:");
                let x = read_player_input();
                println!("enter row:");
                let y = read_player_input();
                if board[x][y].is_none() {
                    board[x][y] = Some(Piece::X);
                    break 'input;
                } else {
                    println!("Invalid move, choose an unoccupied square!");
                }
            }
        } else {
            make_computers_move(&mut board);
        }
        if let Some(p) = evaluate_board(&board) {
            print_board(board);
            println!("{}", match p {
                Piece::X => "You win!",
                Piece::O => "Computer wins!"
            });
            std::process::exit(0)
        }
        turn += 1;
    }
    print_board(board);
    println!("DRAW!");
}
