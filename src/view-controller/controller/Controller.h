//
// Created by marcosrolando on 4/3/21.
//

#ifndef TETRIS_CONTROLLER_H
#define TETRIS_CONTROLLER_H

#include "../../common/GameState.h"

struct Window;

typedef struct Controller {
    int foo; //todo
} Controller_t;

void controller_init(Controller_t* this);

Input_t controller_read_event(Controller_t* this, struct Window* screen);

void controller_release(Controller_t* this);

#endif //TETRIS_CONTROLLER_H
