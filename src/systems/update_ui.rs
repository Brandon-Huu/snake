use crate::systems::ScoreChangeEvent;
use crate::ScoreTextField;
use bevy::prelude::*;

pub fn update_score_text(
    mut event_score_change: EventReader<ScoreChangeEvent>,
    mut score_text: Query<&mut Text, With<ScoreTextField>>,
) {
    for score in event_score_change.read() {
        let mut text = score_text.single_mut();
        text.sections[0].value = format!("Score: {}", score.0);
    }
}
