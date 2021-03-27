mod view_unit;
mod pieces;
mod board;

use view_unit::ViewUnit;
use board::Board;
use crate::pieces::orange_ricky::OrangeRicky;
use crate::pieces::hero::Hero;

fn main() {
    unsafe {
        let mut view_unit = ViewUnit {viewer: std::ptr::null_mut()};
        view_unit::viewUnit_init(&mut view_unit);
        view_unit::viewUnit_render(&view_unit);
        view_unit::viewUnit_release(&mut view_unit);
    }
    let board = Board::new_default();
    let ricky = OrangeRicky {x: 3};
    let hero = Hero {x: 4};
    board.check_collision(&hero);
}
