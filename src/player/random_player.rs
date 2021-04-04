use crate::player::player::Player;

use rand::Rng;

pub struct RandomPlayer {
    pub name: String,
}

impl RandomPlayer {
}

impl Player for RandomPlayer {
    fn new(name: String) -> Self {
        Self{name: name}
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn make_move(&self, board: &Vec<Vec<String>>) -> (usize, usize) {
        let n = board.len();
        loop {
            let mut rng = rand::thread_rng();
            let ind = rng.gen_range(0..n*n);
            let (x, y) = (ind%3, ind/3);
            if board[y][x].eq(" ") {
                println!("RandomPlayer made a move {}", ind);
                return (x, y)
            }

        }

    }
}


