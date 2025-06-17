pub mod entities;
pub mod gamestate;
pub mod updatable;

use crate::gamestate::GameState;
use tetra::ContextBuilder;

const WINDOW_WIDTH: i32 = 740;
const WINDOW_HEIGHT: i32 = 480;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
        .quit_on_escape(true)
        .show_mouse(false)
        .build()?
        .run(|ctx| GameState::new(ctx, "resources/"))
}
