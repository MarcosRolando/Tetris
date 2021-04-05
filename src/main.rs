mod game_engine;
mod model;

use crate::model::game::Game;
use crate::model::pieces::piece::{Movement, Rotation};
use std::thread;
use std::time::{Duration, Instant};
use crate::game_engine::{GameState, PieceTile_t, GameEngine_t, gameEngine_render, gameEngine_release,
                         gameEngine_init, gameEngine_read_event};

fn main() {
    unsafe {
        let mut game = Game::new_default();
        const FRAME_TIME: f32 = 1.0 / 60.0; //in seconds
        let mut start = Instant::now();
        let mut now= start;
        let mut update_duration: f32;
        let mut engine = GameEngine_t {
            controller: std::ptr::null_mut(),
            gui: std::ptr::null_mut(),
            screen: std::ptr::null_mut(),
        }; //todo ver de que no pueda ver estas cosas!!
        let s = gameEngine_init(&mut engine);
        if s != 0 { panic!("Error in gameEngine_init!"); }
        loop {
            let e = gameEngine_read_event(&engine);
            game.update(e);
            let game_state = game.get_state();
            //game.print();
            gameEngine_render(&engine, &game_state);
            now = Instant::now();
            update_duration = (now - start).as_secs_f32();
            if FRAME_TIME > update_duration {
                thread::sleep(Duration::from_secs_f32(FRAME_TIME - update_duration));
            }
            start = Instant::now();
        }
        gameEngine_release(&mut engine);
    }
}
