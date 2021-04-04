use super::{
    J_piece::JPiece,
    L_piece::LPiece,
    I_piece::IPiece,
    T_piece::TPiece,
    O_piece::OPiece,
    S_piece::SPiece,
    Z_piece::ZPiece,
};
use rand::{distributions::{Distribution, Standard}, Rng, random};
use super::piece::PieceType;
use crate::model::pieces::piece::{Piece, Position};

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
    pub fn new(starting_position: Position) -> Piece<dyn PieceType> {
        let piece_type: PieceTypeID = random();
        match piece_type {
            PieceTypeID::Hero => Piece::new(starting_position, Box::new(IPiece {})),
            PieceTypeID::OrangeRicky => Piece::new(starting_position, Box::new(LPiece {})),
            PieceTypeID::BlueRicky => Piece::new(starting_position, Box::new(JPiece {})),
            PieceTypeID::Teewee => Piece::new(starting_position, Box::new(TPiece {})),
            PieceTypeID::ClevelandZ => Piece::new(starting_position, Box::new(ZPiece {})),
            PieceTypeID::RhodeIslandZ => Piece::new(starting_position, Box::new(SPiece {})),
            PieceTypeID::Smashboy => Piece::new(starting_position, Box::new(OPiece {})),
        }
    }
}