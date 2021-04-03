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
use crate::view_unit::{GameState, PieceTile_t, viewUnit_read_event, Input_DOWN, Input_RIGHT, Input_LEFT};
use crate::board::Board;

fn main() {
    unsafe {
        //let stdin_channel = spawn_stdin_channel();
        let mut game = Game::new_default();
        const FRAME_TIME: u128 = (1000 / 60) as u128;
        let mut start = Instant::now();
        let mut now= Instant::now();
        let mut update_duration: u128;
        let mut view_unit = ViewUnit_t { viewer: std::ptr::null_mut() };
        let s = view_unit::viewUnit_init(&mut view_unit);
        if s != 0 { panic!("Error in ViewUnit_init!"); }
        loop {
            let e = viewUnit_read_event(&view_unit);
            if e == Input_DOWN {
                game.move_piece(Movement::Down); //todo por algun motivo si uso match chequea el tipo de dato y no el dato en si! VER!!
            } else if e == Input_RIGHT {
                game.move_piece(Movement::Right);
            } else if e == Input_LEFT {
                game.move_piece(Movement::Left);
            }
            game.update((now - start).as_secs_f32());
            let game_state = game.get_state();
            start = Instant::now();
            view_unit::viewUnit_render(&view_unit, &game_state); //todo por algun motivo llamar a esta funcion rompe la actualizacion del juego
            //game.print();
            now = Instant::now();
            update_duration = (now - start).as_millis();
            /*
            if update_duration <= FRAME_TIME {
                thread::sleep(Duration::from_millis((FRAME_TIME - update_duration) as u64));
            }
            */ //todo no hace falta porque ya uso VSYNC
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
