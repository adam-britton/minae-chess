mod board;
use board::Board;

fn main() {
    let starting_board = Board::from_starting_position();
    println!("{}", starting_board);

    let e2e4_board = Board::from_fen(&String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"));
    println!("{}", e2e4_board);
}
