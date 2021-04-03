mod game_engine;
mod model;

use crate::model::game::Game;
use crate::model::pieces::piece::{Movement, Rotation};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::mpsc;
use std::io::Read;
use raw_tty::IntoRawMode;
use crate::game_engine::{GameState, PieceTile_t, GameEngine_t, gameEngine_render, gameEngine_release, gameEngine_init, gameEngine_read_event, INPUT_DOWN, INPUT_RIGHT, INPUT_LEFT};

fn main() {
    unsafe {
        //let stdin_channel = spawn_stdin_channel();
        let mut game = Game::new_default();
        const FRAME_TIME: u128 = (1000 / 60) as u128;
        let mut start = Instant::now();
        let mut now= Instant::now();
        let mut update_duration: u128;
        let mut engine = GameEngine_t {
            controller: std::ptr::null_mut(),
            gui: std::ptr::null_mut(),
            screen: std::ptr::null_mut(),
        }; //todo ver de que no pueda ver estas cosas!!
        let s = gameEngine_init(&mut engine);
        if s != 0 { panic!("Error in gameEngine_init!"); }
        loop {
            let e = gameEngine_read_event(&engine);
            if e == INPUT_DOWN {
                game.move_piece(Movement::Down); //todo por algun motivo si uso match chequea el tipo de dato y no el dato en si! VER!!
            } else if e == INPUT_RIGHT {
                game.move_piece(Movement::Right);
            } else if e == INPUT_LEFT {
                game.move_piece(Movement::Left);
            }
            game.update((now - start).as_secs_f32());
            let game_state = game.get_state();
            start = Instant::now();
            gameEngine_render(&engine, &game_state);
            //game.print();
            now = Instant::now();
            update_duration = (now - start).as_millis();
            /*
            if update_duration <= FRAME_TIME {
                thread::sleep(Duration::from_millis((FRAME_TIME - update_duration) as u64));
            }
            */ //todo no hace falta porque ya uso VSYNC
        }
        gameEngine_release(&mut engine);
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
