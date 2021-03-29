use crate::player::player::Player;
use crate::player::random_player::RandomPlayer;
use crate::player::human_player::HumanPlayer;

const PLAYER_NAME: [char; 2] = ['X', 'O'];

pub struct GameDriver {
    size: usize,
    board: Vec<Vec<char>>,
    players: Vec<Box<dyn Player>>
}

impl GameDriver {
    pub fn new(size:usize) -> Self {
        Self {
            size: size,
            board: vec![vec![' '; size]; size],
            players: Vec::<Box<dyn Player>>::new()
        }
    }

    pub fn start(&mut self) {
        self.add_player();
    }

    fn add_player(&mut self) {
        for i in 0..2 {
            let mut readOK = false;
            while !readOK {
                println!("Add player {}, 'c' for computer, 'h' for human", i+1);
                let mut playertype = String::new();
                let tmp = std::io::stdin().read_line(&mut playertype);
                if tmp.is_err() {
                    println!("Error reading from input");
                } else {
                    match playertype.trim() {
                        "c" => {
                            println!("Entered {}, adding RandomPlayer for player {}", playertype.trim(), i+1);
                            self.players.push(Box::new(RandomPlayer{name: &PLAYER_NAME[i]}));
                            readOK = true;
                        }
                        "h" => {
                            println!("Entered {}, adding HumanPlayer for player {}", playertype.trim(), i+1);
                            self.players.push(Box::new(HumanPlayer{name: &PLAYER_NAME[i]}));
                            readOK = true;
                        }
                        _ => {
                            println!("Invalied input {}", playertype);
                        }
                    }
                }
            }
        }
    }


}
