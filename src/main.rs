// #![warn(missing_docs)]
#[macro_use]
extern crate log;
use flexi_logger::{
    colored_opt_format, FileSpec, FlexiLoggerError, Logger, LoggerHandle, WriteMode,
};

use game::Game;

pub mod core;
pub mod examples;
pub mod game;

const WINDOW_NAME: &str = "Rusty Engine";
const LOG_FILE_NAME: &str = "log/app.log";
const LOG_LEVEL: &str = "info";

// these have to be integer mults of each other
// in order to have proper cell division in positions
const WINDOW_WIDTH: i32 = 1536;
const WINDOW_HEIGHT: i32 = 864;
const VIRTUAL_RESOLUTION_X: i32 = 512;
const VIRTUAL_RESOLUTION_Y: i32 = 288;

fn start_logger(log_level: &str) -> Result<LoggerHandle, FlexiLoggerError> {
    let logger = Logger::try_with_str(log_level)?
        .log_to_file(FileSpec::try_from(LOG_FILE_NAME)?)
        .write_mode(WriteMode::BufferAndFlush)
        .format(colored_opt_format)
        .start()?;

    Ok(logger)
}

fn main() {
    let _logger = start_logger(LOG_LEVEL);

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
