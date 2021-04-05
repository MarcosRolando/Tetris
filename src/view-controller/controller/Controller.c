//
// Created by marcosrolando on 4/3/21.
//

#include "Controller.h"
#include "SDL.h"
#include "../view/Window.h"

/*
 * PRIVATE
 */

Input_t _case_key_down(const Controller_t* this, const SDL_Event* event) {
    if (event->key.repeat == 0) { //The player started holding down the key
        switch (event->key.keysym.sym) {
            case SDLK_s:
                return INPUT_HOLD_DOWN;
            case SDLK_d:
                return INPUT_HOLD_RIGHT;
            case SDLK_a:
                return INPUT_HOLD_LEFT;
            case SDLK_e:
                return INPUT_ROTATE_RIGHT;
            case SDLK_q:
                return INPUT_ROTATE_LEFT;
        }
    }
    return INPUT_NONE;
}

Input_t _case_key_up(const Controller_t* this, const SDL_Event* event) {
    switch (event->key.keysym.sym) {
        case SDLK_s:
            return INPUT_RELEASE_DOWN;
        case SDLK_d:
            return INPUT_RELEASE_RIGHT;
        case SDLK_a:
            return INPUT_RELEASE_LEFT;
    }
    return INPUT_NONE;
}

/*
 * PUBLIC
 */

void controller_init(Controller_t* this) {

}

Input_t controller_read_event(Controller_t* this, Window_t* screen) {
    SDL_Event event;
    if (SDL_PollEvent(&event)) {
        if (!window_handle_event(screen, &event)) {
            switch (event.key.type) {
                case SDL_KEYDOWN:
                    return _case_key_down(this, &event);
                case SDL_KEYUP:
                    return _case_key_up(this, &event);
            }
        }
    }
    return INPUT_NONE;
}

void controller_release(Controller_t* this) {

}
