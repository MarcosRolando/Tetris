//
// Created by marcosrolando on 4/2/21.
//

#ifndef TETRIS_MEMALLOCMACRO_H
#define TETRIS_MEMALLOCMACRO_H

/* This macro exists with the sole porpose of not having to fucking code the same shit for heap allocation errors */

#define CHECK_ALLOC_RESULT(data, message) \
    if (!data) {                         \
        fprintf(stderr, message); \
        exit(1); \
    }

#endif //TETRIS_MEMALLOCMACRO_H
