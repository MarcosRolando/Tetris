mod view_unit;
mod pieces;
mod game;
mod board;

use view_unit::ViewUnit;
use game::Game;
use crate::pieces::piece::{Piece, Position, Movement};
use crate::pieces::orange_ricky::OrangeRicky;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::mpsc;
use std::io::Read;
use std::process::Command;

fn main() {
/*
    unsafe {
        let mut view_unit = ViewUnit {viewer: std::ptr::null_mut()};
        view_unit::viewUnit_init(&mut view_unit);
        view_unit::viewUnit_render(&view_unit);
        view_unit::viewUnit_release(&mut view_unit);
    }
 */
    let stdin_channel = spawn_stdin_channel();
    let mut game = Game::new_default();
    let frame_time = (1 / 60) as f32; //16.6 ms, game runs at 60fps
    loop {
        match stdin_channel.try_recv() {
            Ok(key) => {
                match key as char {
                    'd' => game.move_piece(Movement::Right),
                    'a' => game.move_piece(Movement::Left),
                    's' => game.move_piece(Movement::Down),
                    _ => (),
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        game.update(frame_time*1000.0);
        game.print();
        thread::sleep(Duration::from_millis((0.5*1000.0) as u64));
    }
}

fn spawn_stdin_channel() -> Receiver<u8> {
    let (sender, receiver) = mpsc::channel::<u8>();
    thread::spawn(move || loop {
        let mut buffer = vec![0;1];
        std::io::stdin().read(&mut buffer).unwrap();
        sender.send(buffer[0]).unwrap();
    });
    receiver
}


