pub trait Player {
    fn new(name: String) -> Self where Self: Sized;
    fn name(&self) -> &String;
    fn make_move(&self, board: &Vec<Vec<String>>) -> (usize, usize);
}
