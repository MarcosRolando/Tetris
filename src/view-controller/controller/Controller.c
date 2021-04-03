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
                    case SDLK_RIGHT:
                        return INPUT_RIGHT;
                    case SDLK_LEFT:
                        return INPUT_LEFT;
                    case SDLK_DOWN:
                        return INPUT_DOWN;
                }
            }
        }
    }
    return INPUT_NONE;
}

void controller_release(Controller_t* this) {

}
