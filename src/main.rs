mod game; // Import the game module

use game::{MainState, WINDOW_WIDTH, WINDOW_HEIGHT};
use ggez::{GameResult, conf, event}; // Import the MainState struct from the game module

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Rusty Pong", "M@ano")
        .window_setup(conf::WindowSetup::default().title("Rusty Pong"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .unwrap();

    let state = MainState::new();
    event::run(ctx, event_loop, state);
    // Ok(())
}
