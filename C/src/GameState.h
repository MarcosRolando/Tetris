//
// Created by marcosrolando on 4/3/21.
//

#ifndef TETRIS_GAMESTATE_H
#define TETRIS_GAMESTATE_H

#define BOARD_WIDTH 10
#define BOARD_HEIGHT 20

/* This module acts as a data crate between the Rust Tetris model and the C viewer. Basically it's the one responsible
 * for comunicating the game state (board configuration, current level, lines cleared, score, etc) to the Viewer */

typedef enum PieceTile {
    HERO,
    SMASHBOY,
    TEEWEE,
    ORANGE_RICKY,
    BLUE_RICKY,
    CLEVELAND_Z,
    RHODE_ISLAND_Z,
    NONE
} PieceTile_t;

typedef struct GameState {
    PieceTile_t board_config[BOARD_HEIGHT][BOARD_WIDTH]; //the current state of the board
    //todo agregar score, info de lineas y level, etc
} GameState_t;

typedef enum Input {
    DOWN,
    RIGHT,
    LEFT,
    EMPTY, //todo ver de agregar el nombre del enum al principio para evitar problemas de namespace
} Input_t;

#endif //TETRIS_GAMESTATE_H
