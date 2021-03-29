use super::{
    blue_ricky::BlueRicky,
    orange_ricky::OrangeRicky,
    hero::Hero,
    teewee::Teewee,
    smashboy::Smashboy,
    rhode_island_z::RhodeIslandZ,
    cleveland_z::ClevelandZ,
};
use rand::{distributions::{Distribution, Standard}, Rng, random};
use super::piece::PieceType;
use crate::pieces::piece::{Piece, Position};
use crate::game::{BOARD_WIDTH, BOARD_HEIGHT};

const STARTING_POSITION: Position = Position {row: BOARD_HEIGHT / 2, column: BOARD_WIDTH / 2}; //todo ver bien el tema del spawn

#[derive(Clone, Copy, Hash)]
enum PieceTypeID {
    Hero,
    Smashboy,
    Teewee,
    OrangeRicky,
    BlueRicky,
    ClevelandZ,
    RhodeIslandZ,
}

/* Returns a random PieceTypeID value, for selecting the next piece */
impl Distribution<PieceTypeID> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceTypeID {
        match rng.gen_range(0..=6) {
            0 => PieceTypeID::Hero,
            1 => PieceTypeID::Smashboy,
            2 => PieceTypeID::Teewee,
            3 => PieceTypeID::OrangeRicky,
            4 => PieceTypeID::BlueRicky,
            5 => PieceTypeID::ClevelandZ,
            _ => PieceTypeID::RhodeIslandZ,
        }
    }
}

pub struct PieceFactory {}

impl PieceFactory {
    /*
    PUBLIC
     */

    /* Returns a random new piece */
    pub fn new() -> Piece<dyn PieceType> {
        let piece_type: PieceTypeID = random();
        match piece_type {
            PieceTypeID::Hero => Piece::new(STARTING_POSITION, Box::new(Hero{})),
            PieceTypeID::OrangeRicky => Piece::new(STARTING_POSITION, Box::new(OrangeRicky{})),
            PieceTypeID::BlueRicky => Piece::new(STARTING_POSITION, Box::new(BlueRicky{})),
            PieceTypeID::Teewee => Piece::new(STARTING_POSITION, Box::new(Teewee{})),
            PieceTypeID::ClevelandZ => Piece::new(STARTING_POSITION, Box::new(ClevelandZ{})),
            PieceTypeID::RhodeIslandZ => Piece::new(STARTING_POSITION, Box::new(RhodeIslandZ{})),
            PieceTypeID::Smashboy => Piece::new(STARTING_POSITION, Box::new(Smashboy{})),
        }
    }
}