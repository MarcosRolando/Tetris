//
// Created by marcosrolando on 3/27/21.
//

#ifndef TETRIS_VIEWUNIT_H
#define TETRIS_VIEWUNIT_H

#include "GameState.h"

/*
 * This unit acts as a translator betweern the Rust code and the C code, the latter being the one
 * who handles all graphics related aspects.
 */

struct Viewer;

typedef struct ViewUnit {
    struct Viewer* viewer; //It's a pointer because if not then bindgen breaks when trying to translae ViewUnit.h to
                        //view_unit.rs because I would have to include Viewer.h which also includes SDL.
} ViewUnit_t;

/* Constructor */
int viewUnit_init(ViewUnit_t* this);

/* Renders the frame based on the current state of the game */
void viewUnit_render(const ViewUnit_t* this, const GameState_t* game_state);

/* Destructor */
void viewUnit_release(ViewUnit_t* this);

#endif //TETRIS_VIEWUNIT_H
