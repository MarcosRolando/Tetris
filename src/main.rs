mod view_unit;
mod pieces;
mod game;
mod board;

use view_unit::ViewUnit_t;
use game::Game;
use crate::pieces::piece::{Movement, Rotation};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::mpsc;
use std::io::Read;
use raw_tty::IntoRawMode;
use crate::view_unit::{GameState, PieceTile_t};
use crate::board::Board;

fn main() {
    unsafe {
        //let stdin_channel = spawn_stdin_channel();
        let mut game = Game::new_default();
        const FRAME_TIME: u128 = (1000 / 60) as u128;
        let mut start = Instant::now();
        let mut now;
        let mut update_duration: u128;
        let mut view_unit = ViewUnit_t { viewer: std::ptr::null_mut() };
        let s = view_unit::viewUnit_init(&mut view_unit);
        if s != 0 { panic!("Error in ViewUnit_init!"); }
        loop {
            /*
            match stdin_channel.try_recv() {
                Ok(key) => {
                    match key as char {
                        'd' => game.move_piece(Movement::Right),
                        'a' => game.move_piece(Movement::Left),
                        's' => game.move_piece(Movement::Down),
                        'e' => game.rotate_piece(Rotation::Right),
                        'q' => game.rotate_piece(Rotation::Left),
                        'f' => break,
                        _ => (),
                    }
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }
            */
            now = Instant::now();
            game.update((now - start).as_secs_f32());
            let game_state = game.get_state();
            view_unit::viewUnit_render(&view_unit, &game_state);
            //game.print();
            now = Instant::now();
            update_duration = (now - start).as_millis();
            start = Instant::now();
            if update_duration <= FRAME_TIME {
                thread::sleep(Duration::from_millis((FRAME_TIME - update_duration) as u64));
            }
        }
        view_unit::viewUnit_release(&mut view_unit);
    }
}

fn spawn_stdin_channel() -> Receiver<u8> {
    let (sender, receiver) = mpsc::channel::<u8>();
    thread::spawn(move || loop {
        let mut buffer = vec![0;1];
        std::io::stdin().into_raw_mode().unwrap().read(&mut buffer).unwrap();
        sender.send(buffer[0]).unwrap();
    });
    receiver
}
