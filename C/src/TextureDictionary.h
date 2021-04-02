//
// Created by marcosrolando on 4/2/21.
//

#ifndef TETRIS_TEXTUREDICTIONARY_H
#define TETRIS_TEXTUREDICTIONARY_H

#include "Texture.h"
#include "TextureID.h"

#define DICT_SIZE 32

struct Node;

typedef struct TextureDictionary {
    struct Node* buckets[DICT_SIZE]; // a dictionary of 32 buckets
} TextureDictionary_t;

void textureDictionary_init(TextureDictionary_t* this);

/* Adds a value for the given key. If the key already exists then it replaces the old value with the value given */
void textureDictionary_add(TextureDictionary_t* this, TextureID key, const Texture_t* value);

/* Returns NULL if the key is not present in the dictionary */
const Texture_t* textureDictionary_at(const TextureDictionary_t* this, TextureID key);

void textureDictionary_release(TextureDictionary_t* this);

#endif //TETRIS_TEXTUREDICTIONARY_H
