mod apple_collision;
mod despawn_segments;
mod game_over;
mod handle_input;
mod move_player;
mod spawn_segments;

pub use game_over::game_over;

pub use apple_collision::apple_collision;

pub use despawn_segments::{GameOverEvent, despawn_segments};
pub use handle_input::handle_input;
pub use move_player::move_player;
pub use spawn_segments::spawn_segments;
