mod view_unit;
mod pieces;
mod game;

use view_unit::ViewUnit;
use game::Game;
use crate::pieces::piece::{Piece, Position};
use crate::pieces::hero::Hero;
use crate::pieces::orange_ricky::OrangeRicky;
use std::borrow::Borrow;

fn main() {

    unsafe {
        let mut view_unit = ViewUnit {viewer: std::ptr::null_mut()};
        view_unit::viewUnit_init(&mut view_unit);
        view_unit::viewUnit_render(&view_unit);
        view_unit::viewUnit_release(&mut view_unit);
    }
    let game = Game::new_default();
    let piece = Piece::new(Position {row:0,column:0}, Box::new(OrangeRicky{}));
    game.update(5);
}

