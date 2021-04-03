//
// Created by marcosrolando on 4/2/21.
//

#include "GUI.h"

int GUI_init(GUI_t* this, SDL_Renderer* renderer) {
    int s = textureRepository_init(&this->texture_repo, renderer);
    if (s) return s;
    board_init(&this->board, &this->texture_repo);
    return 0;
}

void GUI_render(const GUI_t* this, const GameState_t* game_state) {
    board_render(&this->board, game_state);
}

void GUI_release(GUI_t* this) {
    board_release(&this->board);
    textureRepository_release(&this->texture_repo);
}
