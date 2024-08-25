use std::io;

fn main() {
    let mut board = Board::new(9, 7);
    loop {
        board.print_state();

        let input = get_input("Enter a column to place your token...");
        let input = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid column");
                continue;
                0
            }
        };
        println!("You entered {}", input);
        if !board.gen_moves().contains(&input) {
            println!("Not a valid move");
            continue;
        }

        board.make_move(&input);
        board.check_win();
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}

struct Board {
    width: u32,
    height: u32,
    side_to_move: u8,
    // 2D representation of our board, where [x, y] is 
    // the token in the corresponding position and the token values
    // 0 = vacant
    // 1 = player 1
    // 2 = player 2
    tokens: Vec<Vec<u8>>,
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        let tokens = vec![vec![0; height as usize]; width as usize];
        Board {
            width,
            height,
            side_to_move: 1,
            tokens,
        }
    }
    fn gen_moves(&self) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        for (x, column) in self.tokens.iter().enumerate() {
            let mut column = column.iter();
            while let Some(token) = column.next_back() {
                if *token > 0 {
                    continue;
                }
                moves.push(x as u8);
                break;
            }
        }
        moves
    }
    fn make_move(&mut self, column_index: &u8) {
        let Some(column) = self.tokens.get_mut(*column_index as usize) else {
            return;
        };
        let mut column = column.iter_mut();
        while let Some(token) = column.next_back() {
            if *token == 0 {
                *token = self.side_to_move;
                self.side_to_move = if self.side_to_move == 1 { 2 } else { 1 };
                return;
            }
        }
    }
    fn check_win(&self) -> u8 {
        // TODO
        0
    }
    fn print_state(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("| {} ", self.tokens[x as usize][y as usize]);
            }
            println!("|");
        }
    }
}
