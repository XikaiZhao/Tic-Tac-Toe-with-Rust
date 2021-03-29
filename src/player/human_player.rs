use crate::player::player::Player;

pub struct HumanPlayer {
    pub name: &'static char,
}

impl HumanPlayer {
}

impl Player for HumanPlayer {
    fn new(name: &'static char) -> Self { 
        Self{name: name}
    }

    fn make_move(&self) -> u32 {
        2u32
    }
}
