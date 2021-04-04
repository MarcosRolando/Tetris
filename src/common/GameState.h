//
// Created by marcosrolando on 4/3/21.
//

#ifndef TETRIS_GAMESTATE_H
#define TETRIS_GAMESTATE_H

#define BOARD_WIDTH 10
#define BOARD_HEIGHT 20

/* This module acts as a data crate between the Rust Tetris model and the view-controller view. Basically it's the one responsible
 * for comunicating the game state (board configuration, current level, lines cleared, score, etc) to the view and
 * the input of the player to the model */

typedef enum PieceTile {
    PIECETILE_I,
    PIECETILE_O,
    PIECETILE_T,
    PIECETILE_L,
    PIECETILE_J,
    PIECETILE_Z,
    PIECETILE_S,
    PIECETILE_NONE
} PieceTile_t;

typedef struct GameState {
    PieceTile_t board_config[BOARD_HEIGHT][BOARD_WIDTH]; //the current state of the board
    //todo agregar score, info de lineas y level, etc
} GameState_t;

typedef enum Input {
    INPUT_DOWN,
    INPUT_RIGHT,
    INPUT_LEFT,
    INPUT_R_RIGHT, //rotate right
    INPUT_R_LEFT, //rotate left
    INPUT_NONE,
} Input_t;

#endif //TETRIS_GAMESTATE_H
