//
// Created by marcosrolando on 4/1/21.
//

#include "Vector.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/*
 * PRIVATE
 */

void _resize(Vector_t* this) {
    uint32_t new_capacity = this->capacity * 2;
    void** new_data = (void**)calloc(new_capacity, sizeof(void*));
    if (!new_data) {
        fprintf(stderr, "Failed to allocate memory for the Vector!\n");
        exit(-1);
    }
    memcpy(new_data, this->data, sizeof(void*)*this->capacity);
    free(this->data);
    this->data = new_data;
    this->capacity = new_capacity;
}

/*
 * PUBLIC
 */

int vector_init(Vector_t* this, uint32_t element_size, uint32_t capacity) {
    if (element_size <= 0) return VECTOR_INVALID_SIZE;
    if (capacity <= 0) return VECTOR_INVALID_CAPACITY;
    this->element_size = element_size;
    this->capacity = capacity;
    this->data = (void**)calloc(capacity, sizeof(void*)*capacity);
    this->curr_elements = 0;
    if (!this->data) {
        fprintf(stderr, "Failed to allocate memory for the Vector!\n");
        exit(-1);
    }
    return 0;
}

void vector_push_back(Vector_t* this, const void* element) {
    if (this->curr_elements == this->capacity) {
        _resize(this);
    }
    void* new_element = (void*)malloc(this->element_size);
    memcpy(new_element, element, this->element_size);
    this->data[this->curr_elements] = new_element;
    this->curr_elements++;
}

void* vector_at(const Vector_t* this, uint32_t position) {
    if (position < 0 || position >= this->curr_elements) return NULL;
    return this->data[position];
}

void vector_release(Vector_t* this) {
    for (uint32_t i = 0; i < this->curr_elements; i++) {
        free(this->data[i]);
    }
    free(this->data);
}
