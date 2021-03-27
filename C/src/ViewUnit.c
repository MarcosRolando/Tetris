//
// Created by marcosrolando on 3/27/21.
//

#include "ViewUnit.h"
#include "Viewer.h"
#include <stdlib.h>
#include <stdio.h>

/*
 * PUBLIC
 */

int view_unit_init(View_Unit_t* this) {
    this->viewer = calloc(1, sizeof(Viewer_t));
    if (!this->viewer) {
        fprintf(stderr, "Failed to allocate memory for the Viewer! Check you RAM usage or try running again\n");
        exit(1);
    }
    int s = viewer_init(this->viewer);
    if (s) free(this->viewer);
    return s;
}

void view_unit_render(const View_Unit_t* this) {
    viewer_render_frame(this->viewer);
}

void view_unit_release(View_Unit_t* this) {
    free(this->viewer);
}