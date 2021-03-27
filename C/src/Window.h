//
// Created by marcosrolando on 3/27/21.
//

#ifndef TETRIS_WINDOW_H
#define TETRIS_WINDOW_H

#include <SDL.h>
#include <stdbool.h>

#define SDL_WINDOW_ERROR 1
#define SDL_RENDERER_ERROR 2

typedef struct Window {
    SDL_Window* window;
    SDL_Renderer* renderer;
    int width, height;
    bool minimized, fullscreen;
} Window_t;

/*
 * Constructor
 * Returns SDL_WINDOW_ERROR if the window failed to be created
 * Returns SDL_RENDERER_ERROR if the renderer failed to be created
 * Otherwise returns 0
*/
int window_init(Window_t* this);

/* Cleans the current frame */
void clear(const Window_t* this);

/* Renders the next frame */
void show(const Window_t* this);

/* Destructor */
void window_release(Window_t* this);

#endif //TETRIS_WINDOW_H
