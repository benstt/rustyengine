use game::Game;

pub mod core;
pub mod game;

const WINDOW_NAME: &str = "Rusty Engine";
const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 768;

fn main() {
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