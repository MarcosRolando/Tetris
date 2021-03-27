mod view_unit;
mod pieces;

use view_unit::View_Unit;
use pieces::hero::Hero;

fn main() {
    /*unsafe {
        let mut view_unit = View_Unit {viewer: std::ptr::null_mut()};
        view_unit::view_unit_init(&mut view_unit);
        view_unit::view_unit_render(&view_unit);
        view_unit::view_unit_release(&mut view_unit);
    }*/
    let x = Hero {x: 3};
}
