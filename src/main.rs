use crate::viewer::Viewer;

mod viewer;

fn main() {
    let mut x = Viewer {
      foo: 3,
    };
    unsafe {
        viewer::viewer_show_frame( &mut x);
    }
}
