use std::io;
use inline_colorization::*;

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

        if board.make_move(&input) {
            println!("Player {} has won!", if board.side_to_move == 1 { 2 } else { 1 });
        }
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
    fn make_move(&mut self, column_index: &u8) -> bool {
        let Some(column) = self.tokens.get_mut(*column_index as usize) else {
            return false;
        };
        let mut column = column.iter_mut();
        let mut y = 0;
        while let Some(token) = column.next_back() {
            if *token == 0 {
                *token = self.side_to_move;
                let y: i32 = (self.height - y).try_into().unwrap();
                let is_win = self.check_win(i32::from(*column_index), y);
                self.side_to_move = if self.side_to_move == 1 { 2 } else { 1 };
                return is_win;
            }
            y += 1;
        }
        false
    }
    fn check_win(&mut self, last_x: i32, last_y: i32) -> bool {
        // To detect a win, we're going to walk in each direction of the piece we just placed
        // until hitting a token of a different type,
        // and if we traverse more than 3 spaces over each direction + its opposite direction, 
        // then we know that we've won
        let directions = [
            (-1, -1), 
            (-1, 0), 
            (-1, 1), 
            (0, -1)];

        for direction in directions {
            let dir_and_invert_dir = [direction, (-direction.0, -direction.1)];
            let mut sum = 0;

            // Look forward, then invert the direction to look backward and take the sum
            for direction in dir_and_invert_dir {
                let mut x = last_x;
                let mut y = last_y;
                loop {
                    x += direction.0;
                    y += direction.1;
                    let Some(column) = self.tokens.get(x as usize) else {
                        break;
                    };
                    let Some(token) = column.get(y as usize) else {
                        break;
                    };
                    if *token != self.side_to_move {
                        break;
                    }
                    sum += 1;
                }
            }
            if sum >= 3 {
                return true; 
            }
        }
        false
    }
    fn print_state(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let token = self.tokens[x as usize][y as usize]; 
                let output = if token == 1 {
                    format!("{color_bright_yellow}o")
                } else if token == 2 {
                    format!("{color_bright_red}x")
                } else {
                    String::from(" ")
                };
                print!("|{output}{color_reset}");
            }
            println!("|");
        }
    }
}
