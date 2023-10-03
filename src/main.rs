mod gamemode;
mod player;
mod obstacle;
mod state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::gamemode::*;
    pub use crate::player::*;
    pub use crate::obstacle::*;
    pub use crate::state::*;
}

use prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;

    main_loop(context, State::new())
}
