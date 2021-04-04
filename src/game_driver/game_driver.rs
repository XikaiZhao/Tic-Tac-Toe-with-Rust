use crate::player::player::Player;
use crate::player::random_player::RandomPlayer;
use crate::player::human_player::HumanPlayer;

const PLAYER_NAME: [char; 2] = ['X', 'O'];

#[derive(PartialEq)]
enum Status {
    CONTINUE,
    FINISHED,
    DRAW
}

pub struct GameDriver {
    size: usize,
    board: Vec<Vec<String>>,
    players: Vec<Box<dyn Player>>,
    num_moves: u32
}

impl GameDriver {
    pub fn new(size:usize) -> Self {
        Self {
            size: size,
            board: vec![vec![" ".to_string(); size]; size],
            players: Vec::<Box<dyn Player>>::new(),
            num_moves: 0u32
        }
    }

    pub fn start(&mut self) {
        self.add_player();
        println!("\n============ Game Started! ============\n");
        self.print_board_index();
        self.make_moves();
        println!("\n============ Game Finished! ============\n");
    }
    
    pub fn restart(&mut self) {
        println!("Want to start a new game? Enter Y/N"); 
        loop {
            let mut r = String::new();
            if let Err(_) = std::io::stdin().read_line(&mut r) {
                println!("Error reading from input, {}", r);
            } else {
                match r.trim() {
                    "Y" => {
                        println!("\n********************");
                        println!("Starting a new game!");
                        println!("********************\n");

                        self.board = vec![vec![" ".to_string(); self.size]; self.size];
                        self.players = Vec::<Box<dyn Player>>::new();
                        self.num_moves = 0u32;
                        self.start();
                    }
                    "N" => {
                        std::process::exit(0x0100);
                    }
                    _ => {
                        println!("Invalied input");
                    }
                }
            }
        }

    }

    fn add_player(&mut self) {
        for i in 0..2 {
            let mut read_ok = false;
            while !read_ok {
                println!("Add player {}, 'c' for computer, 'h' for human", i+1);
                let mut playertype = String::new();
                if let Err(err) = std::io::stdin().read_line(&mut playertype) {
                    println!("Error reading from input, {}", err);
                } else {
                    match playertype.trim() {
                        "c" => {
                            self.players.push(Box::new(RandomPlayer{name: PLAYER_NAME[i].to_string()}));
                            read_ok = true;
                            println!("Entered {}, added RandomPlayer for player {}", playertype.trim(), i+1);
                        }
                        "h" => {
                            self.players.push(Box::new(HumanPlayer{name: PLAYER_NAME[i].to_string()}));
                            read_ok = true;
                            println!("Entered {}, added HumanPlayer for player {}", playertype.trim(), i+1);
                        }
                        _ => {
                            println!("Invalied input {}", playertype);
                        }
                    }
                }
            }
        }
    }

    fn make_moves(&mut self) {
        let mut turn = 0usize;
        loop {
            let this_move = self.players[turn].make_move(&self.board);
            self.num_moves += 1;
            self.board[this_move.1][this_move.0] = self.players[turn].name().to_string();
            self.print_board_index();

            let game_status = self.check_game_status(this_move);
            
            match game_status  {
                Status::DRAW => {
                    println!("Draw!");
                    break;
                }
                Status::FINISHED => {
                    println!("Player {} won!", self.players[turn].name());
                    break;
                }
                _ => {} 
            } 
            
            turn = (turn+1)%self.players.len();
        }
    }


    fn check_game_status(&self, last_move: (usize, usize)) -> Status {
        let x = last_move.0;
        let y = last_move.1;
        let name = &self.board[y][x];
        
        let row_check = (0..self.size).fold (true, |eq, i| {eq&self.board[y][i].eq(name)});
        if row_check {
            return Status::FINISHED
        }
        
        let col_check = (0..self.size).fold (true, |eq, i| {eq&self.board[i][x].eq(name)});
        if col_check {
            return Status::FINISHED
        }
        
        if x == y || x+y+1 == self.size {
            let diag_check = (0..self.size).fold (true, |eq, i| {eq&self.board[i][i].eq(name)})
                || (0..self.size).fold (true, |eq, i| {eq&self.board[i][self.size-1-i].eq(name)});
            if diag_check {
                return Status::FINISHED
            }
        }

        if self.num_moves == 9 { Status::DRAW }
        else { Status::CONTINUE }
    }
    
    #[allow(dead_code)]
    fn print_board(&self) {
        let sep = "+---+---+---+";
        println!("\n{}", sep);

        for i in &self.board {
            println!("| {} |", i.join(" | "));
            println!("{}", sep);
        }

        println!("");
    }
    
    
    #[allow(dead_code)]
    fn print_index(&self) {
        let sep = "+---+---+---+";
        println!("\n{}", sep);

        for i in 0..self.size {
            println!("| {} |", ((i*self.size)..((i+1)*self.size)).map(|n| n.to_string()).collect::<Vec<String>>().join(" | "));
            println!("{}", sep);
        }

        println!("");
    }
    
    fn print_board_index(&self) {
        let sep = "+---+---+---+ \t\t +---+---+---+";
        println!("\n{}", sep);

        for i in 0..self.size {
            println!("| {} |", self.board[i].join(" | ")+" | \t\t | "+&((i*self.size)..((i+1)*self.size)).map(|n| n.to_string()).collect::<Vec<String>>().join(" | "));
            println!("{}", sep);
        }

        println!("");
    }


}
