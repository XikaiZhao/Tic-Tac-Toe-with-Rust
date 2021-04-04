use crate::player::player::Player;

pub struct HumanPlayer {
    pub name: String,
}

impl HumanPlayer {
}

impl Player for HumanPlayer {
    fn new(name: String) -> Self { 
        Self{name: name}
    }
    
    fn name(&self) -> &String {
        &self.name
    }

    fn make_move(&self, board: &Vec<Vec<String>>) -> (usize, usize) {
        let n:usize = board.len();

        let mut input_move = "".to_string();
        println!("Please enter a number 0--{} to make a move", n*n-1);
        if let Err(err) = std::io::stdin().read_line(&mut input_move) {
            println!("Error reading from input, {}", err);
            self.make_move(board)
        } else {
            match input_move.trim().parse::<usize>() {
                Ok(my_move) => {
                    if my_move < n*n {
                        let (x, y) = (my_move%3, my_move/3);
                        if !board[y][x].eq(" ") {
                            println!("Entered {}, invalid move!", my_move);
                            self.make_move(board)
                        } else {
                            println!("HumanPlayer made a move {}", my_move);
                            (x, y)
                        }
                    } else {
                        println!("Invalid input {}", input_move);
                        self.make_move(board)
                    }
                    
                }
                _ =>  {
                    println!("Invalied input {}", input_move);
                    self.make_move(board)
                }
            }
        }
    }
}
