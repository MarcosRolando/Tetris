//
// Created by marcosrolando on 4/3/21.
//

#include "Controller.h"
#include "SDL.h"
#include "../view/Window.h"

void controller_init(Controller_t* this) {

}

Input_t controller_read_event(Controller_t* this, Window_t* screen) {
    SDL_Event event;
    if (SDL_PollEvent(&event)) {
        if (!window_handle_event(screen, &event)) {
            if (event.key.type == SDL_KEYDOWN) {
                switch (event.key.keysym.sym) {
                    case SDLK_d:
                        return INPUT_RIGHT;
                    case SDLK_a:
                        return INPUT_LEFT;
                    case SDLK_s:
                        return INPUT_DOWN;
                    case SDLK_q:
                        return INPUT_R_LEFT;
                    case SDLK_e:
                        return INPUT_R_RIGHT;
                }
            }
        }
    }
    return INPUT_NONE;
}

void controller_release(Controller_t* this) {

}
