#[derive(Copy, Clone, PartialEq)]
enum State {
    Player1,
    Player2,
    Open,
}

impl State {
    fn string(self) -> String {
        match self {
            State::Player1 => "X".to_string(),
            State::Player2 => "O".to_string(),
            State::Open => " ".to_string(),
        }
    }
}

struct TicTacToe {
    board: [State; 9],
    turn: State,
    turns_played: u8,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        let t: TicTacToe = TicTacToe {
            board: [State::Open; 9],
            turn: State::Player1,
            turns_played: 0,
        };
        t
    }
    fn place_move(&mut self, row: u8, col: u8) -> Result<(), String> {
        match row {
            0..=2 => {}
            _ => return Err(format!("Invalid Row!")),
        };
        match col {
            0..=2 => {}
            _ => return Err(format!("Invalid Col!")),
        };
        match self.board[(row * 3 + col) as usize] {
            State::Open => {}
            _ => return Err(format!("Position ({}, {}) already taken.", row, col)),
        };

        self.board[(row * 3 + col) as usize] = self.turn;
        self.turns_played += 1;

        self.turn = match self.turn {
            State::Player1 => State::Player2,
            State::Player2 => State::Player1,
            State::Open => State::Player1,
        };
        Ok(())
    }
    fn game_over(&self) -> Option<State> {
        if self.turns_played == 9 {
            return Some(State::Open);
        }

        // Check rows/cols
        let b = self.board;
        for i in 0..3 {
            if b[(i * 3) + 0] != State::Open
                && b[(i * 3) + 0] == b[(i * 3) + 1]
                && b[(i * 3) + 1] == b[(i * 3) + 2]
            {
                return Some(b[(i + 3) + 0]);
            }
            if b[(0 * 3) + i] != State::Open
                && b[(0 * 3) + i] == b[(1 * 3) + i]
                && b[(1 * 3) + i] == b[(2 * 3) + i]
            {
                return Some(b[(0 * 3) + i]);
            }
        }
        // Check diagonals
        if b[0] != State::Open && b[0] == b[4] && b[4] == b[8] {
            return Some(b[0]);
        }
        if b[2] != State::Open && b[2] == b[4] && b[4] == b[6] {
            return Some(b[0]);
        }

        None
    }
    fn print_board(&self) {
        println!("Board:");
        for i in 0..9 {
            print!("{}", self.board[i].string());
            if (i + 1) % 3 != 0 {
                print!("|");
            }

            if (i + 1) % 3 == 0 && i != 8 {
                println!("\n-----");
            }
        }
        println!();
    }
}

fn main() {
    let mut t = TicTacToe::new();
    t.print_board();

    loop {
        // Collect input
        println!("Input next move for player \"{}\":", t.turn.string());
        let mut input = String::new();
        let status = std::io::stdin().read_line(&mut input);
        match status {
            Ok(n) => n,
            Err(error) => {
                println!("Error reading line: {}", error);
                continue;
            }
        };
        let parts: Vec<&str> = input.trim().split(" ").collect();
        match parts.len() {
            2 => {}
            _ => {
                println!("Bad user input.");
                continue;
            }
        }

        // Parse Row and Column input
        let mut row = match parts[0].parse::<u8>() {
            Ok(n) => n,
            Err(error) => {
                println!("Row parsing failed. {}", error);
                continue;
            }
        };
        row -= 1;
        let mut col = match parts[1].parse::<u8>() {
            Ok(n) => n,
            Err(error) => {
                println!("Column parsing failed. {}", error);
                continue;
            }
        };
        col -= 1;

        // Place the move
        print!("Placing move at ({}, {}):\n", row, col);
        match t.place_move(row, col) {
            Ok(_) => {}
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        };

        // Show state
        t.print_board();

        // Check win state
        match t.game_over() {
            Some(game_state) => {
                match game_state {
                    State::Player1 => {
                        println!("Game Over. Player 1 wins!");
                    }
                    State::Player2 => {
                        println!("Game Over. Player 2 wins!");
                    }
                    State::Open => {
                        println!("Game Over. Cat's game!");
                    }
                }
                break;
            }
            None => {}
        }
    }
}
