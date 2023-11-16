use crate::Score;
use bevy::prelude::*;

#[derive(Event)]
pub struct AddPointEvent(pub u64);
#[derive(Event)]
pub struct ResetScoreEvent {}

#[derive(Event, Debug)]
pub struct ScoreChangeEvent(pub u64);

pub fn add_points(
    mut event_score_change: EventWriter<ScoreChangeEvent>,
    mut event_add_points: EventReader<AddPointEvent>,
    mut score: ResMut<Score>,
) {
    for points in event_add_points.read() {
        score.0 += points.0;
        event_score_change.send(ScoreChangeEvent(score.0));
    }
}

pub fn reset_score(
    mut event_score_change: EventWriter<ScoreChangeEvent>,
    mut event_reset_score: EventReader<ResetScoreEvent>,
    mut score: ResMut<Score>,
) {
    for _ in event_reset_score.read() {
        score.0 = 0;
        event_score_change.send(ScoreChangeEvent(0));
    }
}
