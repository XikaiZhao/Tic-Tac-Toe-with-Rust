pub trait Player {
    fn new(name: &'static char) -> Self where Self: Sized;
    fn make_move(&self) -> u32;
}
