mod game_engine;
mod model;

use crate::model::game::game::Game;
use std::thread;
use std::time::{Duration, Instant};
use crate::game_engine::{GameEngine_t};

fn main() {
    let mut game = Game::new_default();
    const FRAME_TIME: f32 = 1.0 / 60.0; //in seconds
    let mut start = Instant::now();
    let mut now: Instant;
    let mut update_duration: f32;
    let engine = GameEngine_t::new();
    loop {
        let e = engine.read_event();
        game.update(e);
        let game_state = game.get_state();
        engine.render(&game_state);
        now = Instant::now();
        update_duration = (now - start).as_secs_f32();
        if FRAME_TIME > update_duration {
            thread::sleep(Duration::from_secs_f32(FRAME_TIME - update_duration));
        }
        start = Instant::now();
    }
}
