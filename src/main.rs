use std::io;
use inline_colorization::*;

const PLAYER_COUNT: u8 = 4;
const CONNECT_AMOUNT: u8 = 3;

fn main() {    
    println!("Welcome to Connect 4");
    loop {
        let _ = get_input("Press enter to begin a match...");
        let mut board = Board::new(13, 11);
        loop {
            board.print_state();
    
            println!("Player {}'s turn!", board.active_player);
            let input = get_input("Enter a column to place your token...");
            let input = match input.parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid column");
                    continue;
                }
            };
            println!("You entered '{}'", input);
            if !board.gen_moves().contains(&input) {
                println!("Not a valid move");
                continue;
            }
    
            if board.make_move(&input) {
                let winner = if board.active_player == 1 { PLAYER_COUNT } else { board.active_player - 1 };
                println!("Player {} has won!", winner);
                board.print_state();
                break;
            }
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
    active_player: u8,
    // 2D representation of our board, where [x, y] is 
    // the token in the corresponding position and the token values
    tokens: Vec<Vec<Token>>,
}

#[derive(Clone)]
enum Token {
    Vacant,
    Owned(u8),
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        let tokens = vec![vec![Token::Vacant; height as usize]; width as usize];
        Board {
            width,
            height,
            active_player: 1,
            tokens,
        }
    }
    fn gen_moves(&self) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        for (x, column) in self.tokens.iter().enumerate() {
            let mut column = column.iter();
            while let Some(token) = column.next_back() {
                let Token::Vacant = token else {
                    continue;
                };
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
            if let Token::Vacant = token {
                *token = Token::Owned(self.active_player);
                let y: i32 = (self.height - y - 1).try_into().unwrap();
                let is_win = self.check_win(i32::from(*column_index), y);
                self.active_player += 1;
                if self.active_player > PLAYER_COUNT {
                    self.active_player = 1;
                }
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
        let offsets = [
            (-1, -1), 
            (-1, 0), 
            (-1, 1), 
            (0, -1)];

        for offset in offsets {
            let offset_and_inverse = [offset, (-offset.0, -offset.1)];
            let mut sum = 0;

            // Look forward, then invert the direction to look backward and take the sum
            for direction in offset_and_inverse {
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
                    match token {
                        Token::Vacant => break,
                        Token::Owned(owner) => {
                            if *owner != self.active_player {
                                break;
                            }
                            sum += 1;
                        }
                    }
                }
            }
            if sum >= CONNECT_AMOUNT - 1 {
                return true;
            }
        }
        false
    }
    fn print_state(&self) {
        for y in 0..self.height {
            print!("{: >2}|", y);
            for x in 0..self.width {
                let token = &self.tokens[x as usize][y as usize];
                let output = match token {
                    Token::Owned(owner) => {
                        match owner {
                            1 => format!("{color_bright_yellow}o"),
                            2 => format!("{color_bright_red}x"),
                            3 => format!("{color_bright_blue}a"),
                            4 => format!("{color_bright_green}n"),
                            _ => String::from(" "),
                        }
                    },
                    _ => String::from(" "),
                };
                print!(" {output}{color_reset} |");
            }
            println!();
        }
        print!("  ");
        for x in 0..self.width {
            print!("|{: ^3}", x);
        }
        println!("|");
    }
}
