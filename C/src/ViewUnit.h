//
// Created by marcosrolando on 3/27/21.
//

#ifndef TETRIS_VIEWUNIT_H
#define TETRIS_VIEWUNIT_H

/*
 * This unit acts as a translator betweern the Rust code and the C code, the latter being the one
 * who handles all graphics related aspects.
 */

struct Viewer;

typedef struct View_Unit {
    struct Viewer* viewer; //It's a pointer because if not then bindgen breaks when trying to translae ViewUnit.h to
                        //view_unit.rs because I would have to include Viewer.h which also includes SDL.
} View_Unit_t;

/* Constructor */
int view_unit_init(View_Unit_t* this);

/* Renders the frame based on the current state of the game */
void view_unit_render(const View_Unit_t* this);

/* Destructor */
void view_unit_release(View_Unit_t* this);

#endif //TETRIS_VIEWUNIT_H
