pub mod structs;

use std::io::Write;

use structs::board::Board;
use structs::mark::Mark;

fn main() {
    let mut board = Board::new();

    'game: loop {
        match board.winner() {
            Some(winner) => {
                if winner != Mark::Empty {
                    println!("Player {winner} won!");
                } else {
                    println!("Tie!");
                }
                println!("{}", board);
                break 'game;
            },
            None => ()
        }

        println!("It's player {}'s turn!\n{}", board.turn(), board);
        'input: loop {
            let mut inp_i: [usize; 2] = [0; 2];

            print!("Move (<row> <col>): ");
            std::io::stdout().flush().unwrap();

            let mut inp = String::new();
            match std::io::stdin().read_line(&mut inp) {
                Ok(_) => (),
                Err(e) => println!("Error while reading line: {e}")
            }

            let args: Vec<&str> = inp.trim().split_ascii_whitespace().collect();
            if args.len() != 2 { continue 'input; }

            for i in 0..2 {
                inp_i[i] = match args.get(i).unwrap().parse() {
                    Ok(val) => if val > 0 { val } else { continue 'input; },
                    Err(_) => continue 'input
                };
            }

            match board.put(inp_i[0] - 1, inp_i[1] - 1) {
                Ok(_) => break 'input,
                Err(_) => continue 'input
            }
        }

        println!();
    }
}
