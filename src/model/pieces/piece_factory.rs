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
    I,
    O,
    T,
    L,
    J,
    Z,
    S,
}

/* Returns a random PieceTypeID value, for selecting the next piece */
impl Distribution<PieceTypeID> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceTypeID {
        let n = rng.gen_range(0..56);
        match rng.gen_range(0..=56) { //The following adhere to the Tetrominoes weights in the classic NES Tetris
                                            // See https://meatfighter.com/nintendotetrisai/?a=b#The_Mechanics_of_Nintendo_Tetris
            0..=6 => PieceTypeID::I,
            7..=14 => PieceTypeID::O,
            15..=23 => PieceTypeID::T,
            24..=30 => PieceTypeID::L,
            31..=38 => PieceTypeID::J,
            39..=46 => PieceTypeID::Z,
            _ => PieceTypeID::S,
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
            PieceTypeID::I => Piece::new(starting_position, Box::new(IPiece {})),
            PieceTypeID::L => Piece::new(starting_position, Box::new(LPiece {})),
            PieceTypeID::J => Piece::new(starting_position, Box::new(JPiece {})),
            PieceTypeID::T => Piece::new(starting_position, Box::new(TPiece {})),
            PieceTypeID::Z => Piece::new(starting_position, Box::new(ZPiece {})),
            PieceTypeID::S => Piece::new(starting_position, Box::new(SPiece {})),
            PieceTypeID::O => Piece::new(starting_position, Box::new(OPiece {})),
        }
    }
}