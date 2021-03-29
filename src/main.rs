mod params;
mod player;
mod game_driver;

fn main() {
    let mut game_driver = game_driver::game_driver::GameDriver::new(params::SIZE);
    println!("start game");
    game_driver.start();
}
