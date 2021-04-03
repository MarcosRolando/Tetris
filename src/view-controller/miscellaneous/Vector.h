//
// Created by marcosrolando on 4/1/21.
//

#ifndef TETRIS_VECTOR_H
#define TETRIS_VECTOR_H

#include <stdint.h>

#define VECTOR_INVALID_SIZE 1
#define VECTOR_INVALID_CAPACITY 2

typedef struct Vector {
    uint32_t element_size; //the size of an element
    uint32_t capacity; //how many elements it can currently hold
    uint32_t curr_elements; //current amount of elements in the Vector
    void** data; //the elements of the Vector
} Vector_t;

int vector_init(Vector_t* this, uint32_t element_size, uint32_t capacity);

/* Adds a new element to the vector, "moving" the element given */
void vector_push_back(Vector_t* this, void* element);

const void* vector_at(const Vector_t* this, uint32_t position);

void vector_release(Vector_t* this);

#endif //TETRIS_VECTOR_H
