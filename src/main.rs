mod view_unit;
mod pieces;
mod board;

use view_unit::View_Unit;
use board::Board;

fn main() {
    /*unsafe {
        let mut view_unit = View_Unit {viewer: std::ptr::null_mut()};
        view_unit::view_unit_init(&mut view_unit);
        view_unit::view_unit_render(&view_unit);
        view_unit::view_unit_release(&mut view_unit);
    }*/
    let x = Board::new_default();
}
