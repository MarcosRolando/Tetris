//
// Created by marcosrolando on 4/2/21.
//

#ifndef TETRIS_GUI_H
#define TETRIS_GUI_H

#include "textures/TextureRepository.h"
#include "Window.h"
#include "Board.h"

typedef struct GUI {
    TextureRepository_t texture_repo;
    Board_t board;
} GUI_t;

int GUI_init(GUI_t* this, SDL_Renderer* renderer);

void GUI_render(const GUI_t* this, const GameState_t* game_state);

void GUI_release(GUI_t* this);

#endif //TETRIS_GUI_H
