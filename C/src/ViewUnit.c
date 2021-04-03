//
// Created by marcosrolando on 3/27/21.
//

#include "ViewUnit.h"
#include "MemAllocMacro.h"
#include "Viewer.h"
#include <stdlib.h>
#include <stdio.h>

/*
 * PUBLIC
 */

int viewUnit_init(ViewUnit_t* this) {
    this->viewer = calloc(1, sizeof(Viewer_t));
    CHECK_ALLOC_RESULT(this->viewer,
                       "Failed to allocate memory for the Viewer! Check you RAM usage or try running again\n")
    int s = viewer_init(this->viewer);
    if (s) free(this->viewer);
    return s;
}

Input_t viewUnit_read_event(const ViewUnit_t* this) {
    return viewer_read_event(this->viewer);
}

void viewUnit_render(const ViewUnit_t* this, const GameState_t* game_state) {
    viewer_render_frame(this->viewer, game_state);
}

void viewUnit_release(ViewUnit_t* this) {
    viewer_release(this->viewer);
    free(this->viewer);
}
