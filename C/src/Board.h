//
// Created by marcosrolando on 4/3/21.
//

#ifndef TETRIS_BOARD_H
#define TETRIS_BOARD_H

#include "GameState.h"

struct TextureRepository;

typedef struct Board {
    const struct TextureRepository* texture_repo;
} Board_t;

void board_init(Board_t* this, const struct TextureRepository* texture_repo);

void board_render(const Board_t* this, const GameState_t* game_state);

void board_release(Board_t* this);

#endif //TETRIS_BOARD_H
