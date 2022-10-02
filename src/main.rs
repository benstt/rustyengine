// #![warn(missing_docs)]
#[macro_use]
extern crate log;
use game::Game;

pub mod core;
pub mod examples;
pub mod game;

const WINDOW_NAME: &str = "Rusty Engine";
const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;
const VIRTUAL_RESOLUTION_X: i32 = 256;
const VIRTUAL_RESOLUTION_Y: i32 = 144;

fn main() {
    env_logger::init();

    info!("Starting the miniquad application");

    miniquad::start(
        miniquad::conf::Conf {
            window_title: WINDOW_NAME.to_string(),
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            window_resizable: true,
            ..Default::default()
        },
        |ctx| Box::new(Game::new(ctx)),
    );
}
