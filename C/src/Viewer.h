//
// Created by marcosrolando on 3/26/21.
//

#ifndef TETRIS_VIEWER_H
#define TETRIS_VIEWER_H

#define SDL_GRAPHICS_ERROR 1
#define SDL_IMAGE_ERROR 2
#define SDL_MIXER_ERROR 3
#define SDL_TTF_ERROR 4

#include "Window.h"

typedef struct Viewer {
    Window_t window;
} Viewer_t;

/*
 * Constructor
 *
 * If there is a graphics error it returns SDL_GRAPHICS_ERROR
 * If there is an image loader error it returns SDL_IMG_ERROR
 * If there is an audio error it returns SDL_MIXER_ERROR
 * If there is a text font loader error it returns SDL_TTF_ERROR
 * Otherwise returns 0
 *
 */
int viewer_init(Viewer_t* this);

void viewer_render_frame(const Viewer_t* this);

/* Destructor */
void viewer_release(Viewer_t* this);


#endif //TETRIS_VIEWER_H
