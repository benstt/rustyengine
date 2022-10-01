#[macro_use]
extern crate log;
use game::Game;

pub mod core;
pub mod examples;
pub mod game;

const WINDOW_NAME: &str = "Rusty Engine";
const WINDOW_WIDTH: i32 = 1408;
const WINDOW_HEIGHT: i32 = 792;

fn main() {
    env_logger::init();

    info!("starting miniquad application");

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
