/* automatically generated by rust-bindgen 0.57.0 */

pub const BOARD_WIDTH: u32 = 10;
pub const BOARD_HEIGHT: u32 = 20;
pub const PIECETILE_HERO: PieceTile = 0;
pub const PIECETILE_SMASHBOY: PieceTile = 1;
pub const PIECETILE_TEEWEE: PieceTile = 2;
pub const PIECETILE_ORANGE_RICKY: PieceTile = 3;
pub const PIECETILE_BLUE_RICKY: PieceTile = 4;
pub const PIECETILE_CLEVELAND_Z: PieceTile = 5;
pub const PIECETILE_RHODE_ISLAND_Z: PieceTile = 6;
pub const PIECETILE_NONE: PieceTile = 7;
pub type PieceTile = ::std::os::raw::c_uint;
pub use self::PieceTile as PieceTile_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub board_config: [[PieceTile_t; 10usize]; 20usize],
}
#[test]
fn bindgen_test_layout_GameState() {
    assert_eq!(
        ::std::mem::size_of::<GameState>(),
        800usize,
        concat!("Size of: ", stringify!(GameState))
    );
    assert_eq!(
        ::std::mem::align_of::<GameState>(),
        4usize,
        concat!("Alignment of ", stringify!(GameState))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GameState>())).board_config as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GameState),
            "::",
            stringify!(board_config)
        )
    );
}
pub type GameState_t = GameState;
pub const INPUT_DOWN: Input = 0;
pub const INPUT_RIGHT: Input = 1;
pub const INPUT_LEFT: Input = 2;
pub const INPUT_R_RIGHT: Input = 3;
pub const INPUT_R_LEFT: Input = 4;
pub const INPUT_NONE: Input = 5;
pub type Input = ::std::os::raw::c_uint;
pub use self::Input as Input_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GUI {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Controller {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Window {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameEngine {
    pub gui: *mut GUI,
    pub controller: *mut Controller,
    pub screen: *mut Window,
}
#[test]
fn bindgen_test_layout_GameEngine() {
    assert_eq!(
        ::std::mem::size_of::<GameEngine>(),
        24usize,
        concat!("Size of: ", stringify!(GameEngine))
    );
    assert_eq!(
        ::std::mem::align_of::<GameEngine>(),
        8usize,
        concat!("Alignment of ", stringify!(GameEngine))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GameEngine>())).gui as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GameEngine),
            "::",
            stringify!(gui)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GameEngine>())).controller as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GameEngine),
            "::",
            stringify!(controller)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GameEngine>())).screen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GameEngine),
            "::",
            stringify!(screen)
        )
    );
}
pub type GameEngine_t = GameEngine;
extern "C" {
    pub fn gameEngine_init(this: *mut GameEngine_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gameEngine_read_event(this: *const GameEngine_t) -> Input_t;
}
extern "C" {
    pub fn gameEngine_render(this: *const GameEngine_t, game_state: *const GameState_t);
}
extern "C" {
    pub fn gameEngine_release(this: *mut GameEngine_t);
}
