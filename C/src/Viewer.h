//
// Created by marcosrolando on 3/26/21.
//

#ifndef TETRIS_VIEWER_H
#define TETRIS_VIEWER_H

#define SDL_GRAPHICS_ERROR 1
#define SDL_IMAGE_ERROR 2
#define SDL_MIXER_ERROR 3
#define SDL_TTF_ERROR 4

typedef struct Viewer {
    int foo; //Relleno por ahora
} Viewer_t;

/* Constructor */
Viewer_t* viewer_init(Viewer_t* this);

void show_frame(Viewer_t* this);

/* Destructor */
void viewer_release(Viewer_t* this);


#endif //TETRIS_VIEWER_H
