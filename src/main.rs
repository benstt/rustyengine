#[macro_use]
extern crate log;
use game::Game;

pub mod core;
pub mod examples;
pub mod game;

const WINDOW_NAME: &str = "Rusty Engine";
const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 768;

fn main() {
    env_logger::init();

    info!("starting miniquad application");

    miniquad::start(
        miniquad::conf::Conf {
            window_title: WINDOW_NAME.to_string(),
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            ..Default::default()
        },
        |ctx| Box::new(Game::new(ctx)),
    );
}
