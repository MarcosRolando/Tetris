mod view_unit;
mod pieces;
mod board;

use view_unit::ViewUnit;
use board::Board;

fn main() {
    unsafe {
        let mut view_unit = ViewUnit {viewer: std::ptr::null_mut()};
        view_unit::viewUnit_init(&mut view_unit);
        view_unit::viewUnit_render(&view_unit);
        view_unit::viewUnit_release(&mut view_unit);
    }
    let x = Board::new_default();
}
