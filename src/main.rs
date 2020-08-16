use board::Board;

mod board;
mod field;

fn main() {
    // let easy = "..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";
    // let hard = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    let s = String::from(
        "......2..\
         .8...7.9.\
         6.2...5..\
         .7..6....\
         ...9.1...\
         ....2..4.\
         ..5...6.3\
         .9.4...7.\
         ..6......",
    );

    let mut board = Board::from_string(s.to_string());

    println!("{}", board);
    println!("is_valid? {}", board.is_valid());
    println!("solved? {}", board.is_solved());

    solve(&mut board);

    println!("{}", board);
    println!("is_valid? {}", board.is_valid());
    println!("solved? {}", board.is_solved());
}

fn solve(board: &mut Board) {
    let mut progress = true;
    while progress {
        progress = board.solve();
        // println!("solve step: \n{}", board);
        if !board.is_valid() {
            // println!("board invalid, return");
            return;
        }
    }

    if board.is_solved() {
        return;
    }

    let mut values = board.possible_values();
    values.sort_by(|a, b| a.1.len().partial_cmp(&b.1.len()).unwrap());

    // println!("{:?}", values);

    let (index, values) = &values[0];

    for v in values.iter() {
        let mut tboard = board.clone();

        // println!("brute update {}/{} {}", index / 9, index % 9, v);
        tboard.update(*index, vec![*v]);

        solve(&mut tboard);

        if tboard.is_solved() && tboard.is_valid() {
            *board = tboard;
            return;
        }
    }
}
