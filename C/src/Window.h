//
// Created by marcosrolando on 3/27/21.
//

#ifndef TETRIS_WINDOW_H
#define TETRIS_WINDOW_H

#include <stdbool.h>
#include <SDL.h>

#define SDL_WINDOW_ERROR 1
#define SDL_RENDERER_ERROR 2

#define DEFAULT_SCREEN_WIDTH 1280
#define DEFAULT_SCREEN_HEIGHT 720

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
void window_clear(const Window_t* this);

/* Renders the next frame */
void window_show(const Window_t* this);

/* Handles window's events, i.e. resizing or minimizing */
bool window_handle_event(Window_t* this, SDL_Event* e);

/* Returns the window's renderer */
SDL_Renderer* window_get_renderer(const Window_t* this);

/* Destructor */
void window_release(Window_t* this);

#endif //TETRIS_WINDOW_H
