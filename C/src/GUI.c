//
// Created by marcosrolando on 4/2/21.
//

#include "GUI.h"

int GUI_init(GUI_t* this) {
    int s = window_init(&this->screen);
    if (s) return s;
    s = textureRepository_init(&this->texture_repo, window_get_renderer(&this->screen));
    if (s) {
        window_release(&this->screen);
        return s;
    }
    board_init(&this->board, &this->texture_repo);
    return 0;
}

Input_t GUI_read_event(GUI_t* this) { //todo reorganizar el controlador y renombrar el tema de ViewUnit y Viewer y eso
    SDL_Event event;
    if (SDL_PollEvent(&event)) {
        if (!window_handle_event(&this->screen, &event)) {
            switch (event.key.keysym.sym) { //valgrind complains about this value not being initialized,
                                            // but this seems to be a valgrind bug vecause PollEvent should initialize it,
                                            // and even when I do it manually with memset it still complains
                case SDLK_RIGHT:
                    return RIGHT;
                case SDLK_LEFT:
                    return LEFT;
                case SDLK_DOWN:
                    return DOWN;
            }
        }
    }
    return EMPTY;
}

void GUI_render(const GUI_t* this, const GameState_t* game_state) {
    window_clear(&this->screen);
    board_render(&this->board, game_state);
    window_show(&this->screen);
}

void GUI_release(GUI_t* this) {
    board_release(&this->board);
    textureRepository_release(&this->texture_repo);
    window_release(&this->screen);
}
