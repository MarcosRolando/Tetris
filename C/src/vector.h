//
// Created by marcosrolando on 4/1/21.
//

#ifndef TETRIS_VECTOR_H
#define TETRIS_VECTOR_H

#include <stdint.h>

#define VECTOR_INVALID_SIZE 1
#define VECTOR_INVALID_CAPACITY 2

typedef struct vector {
    uint32_t element_size; //the size of an element
    uint32_t capacity; //how many elements it can currently hold
    uint32_t curr_elements; //current amount of elements in the vector
    void** data; //the elements of the vector
} vector_t;

int vector_init(vector_t* this, uint32_t element_size, uint32_t capacity);

void vector_push_back(vector_t* this, const void* element);

void* vector_at(const vector_t* this, uint32_t position);

void vector_release(vector_t* this);

#endif //TETRIS_VECTOR_H
