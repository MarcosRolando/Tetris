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

static void _handle_resize_event(Window_t* this, const SDL_Event* e, bool handled) {
    if (e->type == SDL_KEYDOWN && e->key.keysym.sym == SDLK_F1) {
        handled = true;
        this->width = DEFAULT_SCREEN_WIDTH;
        this->height = DEFAULT_SCREEN_HEIGHT;
        if (this->fullscreen) {
            SDL_SetWindowFullscreen(this->window, SDL_FALSE);
            SDL_SetWindowSize(this->window, this->width, this->height);
            this->fullscreen = false;
        } else {
            SDL_SetWindowFullscreen(this->window, SDL_TRUE);
            this->width = 1920;
            this->height = 1080;
            SDL_SetWindowSize(this->window, this->width, this->height);
            this->fullscreen = true;
            this->minimized = false;
        }
    } else if (e->type == SDL_KEYDOWN && e->key.keysym.sym == SDLK_F2) {
        handled = true;
        SDL_SetWindowFullscreen(this->window, SDL_FALSE);
        this->fullscreen = false;
        this->width = DEFAULT_SCREEN_WIDTH;
        this->height = DEFAULT_SCREEN_HEIGHT;
        SDL_RestoreWindow(this->window);
        SDL_SetWindowSize(this->window, this->width, this->height);
    }
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

void window_clear(const Window_t* this) {
    SDL_SetRenderDrawColor(this->renderer, 0xFF, 0xFF, 0xFF, 0xFF);
    SDL_RenderClear(this->renderer);
}

void window_show(const Window_t* this) {
    if (!this->minimized) {
        float x_scale = (float)this->width / (float)DEFAULT_SCREEN_WIDTH;
        float y_scale = (float)this->height / (float)DEFAULT_SCREEN_HEIGHT;
        SDL_RenderSetScale(this->renderer, x_scale, y_scale);
        SDL_RenderPresent(this->renderer);
    }
}

bool window_handle_event(Window_t* this, SDL_Event* e) {
    bool handled = false;
    if (e->type == SDL_WINDOWEVENT) {
        switch (e->window.event) {
            case SDL_WINDOWEVENT_SIZE_CHANGED:
                this->width = e->window.data1;
                this->height = e->window.data2;
                window_show(this);
                break;
            case SDL_WINDOWEVENT_EXPOSED:
                SDL_RenderPresent(this->renderer);
                break;
            case SDL_WINDOWEVENT_FOCUS_GAINED:
            case SDL_WINDOWEVENT_MAXIMIZED:
            case SDL_WINDOWEVENT_RESTORED:
                this->minimized = false;
                break;
        }
        handled = true;
    }
    _handle_resize_event(this, e, handled);
    return handled;
}

void window_release(Window_t* this) {
    if (this->renderer != NULL) SDL_DestroyRenderer(this->renderer);
    if (this->window != NULL) SDL_DestroyWindow(this->window);
}
