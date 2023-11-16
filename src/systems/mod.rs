mod apple_collision;
mod despawn_segments;
mod game_over;
mod handle_input;
mod move_player;
mod score_system;
mod spawn_segments;
mod update_ui;

pub use score_system::{add_points, reset_score, AddPointEvent, ResetScoreEvent, ScoreChangeEvent};
pub use update_ui::update_score_text;

pub use game_over::game_over;

pub use apple_collision::apple_collision;

pub use despawn_segments::{despawn_all, despawn_segments, GameOverEvent};
pub use handle_input::handle_input;
pub use move_player::move_player;
pub use spawn_segments::spawn_segments;
