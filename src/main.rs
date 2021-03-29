mod view_unit;
mod pieces;
mod game;

use view_unit::ViewUnit;
use game::Game;
use crate::pieces::piece::{Piece, Position};
use crate::pieces::orange_ricky::OrangeRicky;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
/*
    unsafe {
        let mut view_unit = ViewUnit {viewer: std::ptr::null_mut()};
        view_unit::viewUnit_init(&mut view_unit);
        view_unit::viewUnit_render(&view_unit);
        view_unit::viewUnit_release(&mut view_unit);
    }
 */
    let mut game = Game::new_default();
    let frame_time = (1 / 60) as f32; //16.6 ms, game runs at 60fps
    loop {
        game.update(frame_time*1000.0);
        game.print();
        thread::sleep(Duration::from_millis((frame_time*1000.0) as u64));
    }
}

