use crate::player::player::Player;

pub struct RandomPlayer {
    pub name: &'static char,
}

impl RandomPlayer {
}

impl Player for RandomPlayer {
    fn new(name: &'static char) -> Self { 
        Self{name: name}
    }

    fn make_move(&self) -> u32 {
        1u32
    }
}


