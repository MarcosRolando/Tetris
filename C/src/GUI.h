//
// Created by marcosrolando on 4/2/21.
//

#ifndef TETRIS_GUI_H
#define TETRIS_GUI_H

#include "TextureRepository.h"
#include "Window.h"

typedef struct GUI {
    Window_t screen;
    TextureRepository_t texture_repo;
} GUI_t;

int GUI_init(GUI_t* this);

void GUI_render(const GUI_t* this);

void GUI_release(GUI_t* this);

#endif //TETRIS_GUI_H
