
use crate::board::board::Board;

fn printBoardFancy(board: &Board) {
    // Note: UCI ignores unrecognized commands, and so we can print by sending junk commands
    // ♔♕♖♗♘♙♚♛♜♝♞♟︎🦆
    let symbols : [&str; 16] = ["♔", "♕", "♖", "♗", "♘", "♙", "🦆", " ", "♚", "♛", "♜", "♝", "♞", "♟︎", "🦆", " "];
    println!("╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗");
    for row in 1..8 {
        let mut row_out : String = String::from("║ ");
        for column in 1..8 {
            row_out = format!("{}{} ║ ", row_out, symbols[board.get_square(8 * row + column).index()]);
        }
        println!("{}", row_out);
        if row != 8 {println!("╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣")}
    }
    println!("╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝")
}

fn genFEN(board: &Board) {

    
}

fn genPGN() {

}