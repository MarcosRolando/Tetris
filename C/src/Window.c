//
// Created by marcosrolando on 3/27/21.
//

#include "Window.h"


/*
 * PRIVATE
 */

static int _create_window(Window_t* this) {
    this->window = SDL_CreateWindow( "TETRIS", SDL_WINDOWPOS_UNDEFINED,
                                SDL_WINDOWPOS_UNDEFINED, DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT,
                                SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE);
    if (this->window != NULL) {
        this->width = DEFAULT_SCREEN_WIDTH;
        this->height = DEFAULT_SCREEN_HEIGHT;
    } else {
        fprintf(stderr, "Window could not be created! Graphics Error: %s\n", SDL_GetError());
        return SDL_WINDOW_ERROR;
    }
    return 0;
}

static int _create_renderer(Window_t* this) {
    this->renderer = SDL_CreateRenderer(this->window, -1, SDL_RENDERER_ACCELERATED
                                               | SDL_RENDERER_PRESENTVSYNC);
    if (this->renderer == NULL) {
        fprintf(stderr, "Renderer could not be created! Graphics Error: %s\n", SDL_GetError());
        return SDL_RENDERER_ERROR;
    }
    SDL_SetRenderDrawColor(this->renderer, 0xFF, 0xFF, 0xFF, 0xFF);
    return 0;
}

/*
 * PUBLIC
 */

int window_init(Window_t* this) {
    this->window = NULL;
    this->renderer = NULL;
    this->fullscreen = false;
    this->minimized = false;
    this->width = 0;
    this->height = 0;
    int s = _create_window(this);
    if (s) return s;
    s = _create_renderer(this);
    return s;
}

void clear(const Window_t* this) {
    SDL_SetRenderDrawColor(this->renderer, 0xFF, 0xFF, 0xFF, 0xFF);
    SDL_RenderClear(this->renderer);
}

void show(const Window_t* this) {
    if (!this->minimized) {
        float x_scale = (float)this->width/(float)DEFAULT_SCREEN_WIDTH;
        float y_scale = (float)this->height/(float)DEFAULT_SCREEN_HEIGHT;
        SDL_RenderSetScale(this->renderer, x_scale, y_scale);
        SDL_RenderPresent(this->renderer);
    }
}

void window_release(Window_t* this) {
    if (this->renderer != NULL) SDL_DestroyRenderer(this->renderer);
    if (this->window != NULL) SDL_DestroyWindow(this->window);
}
