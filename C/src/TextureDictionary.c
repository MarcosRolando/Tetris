//
// Created by marcosrolando on 4/2/21.
//

#include "TextureDictionary.h"
#include <string.h>

typedef struct Node {
    TextureID key;
    struct Node* next;
    Texture_t texture;
} Node_t;

static void _node_init(Node_t* this, TextureID key, Node_t* next, const Texture_t* texture) {
    this->key = key;
    this->next = next;
    this->texture = *texture;
}

static void _node_release(Node_t* this) {
    if (this->next) {
        _node_release(this->next);
        free(this->next);
    }
    texture_release(&this->texture);
}

/*
 * PRIVATE
 */


/* Calculates the hash for a given key. Being that the keys are enum then the optimal hashing function (that is,
 * the one that produces the least amount of collisions) if f(x) = x % DICT_SIZE, so that is the one being used */
static uint32_t _hash_value(TextureID key) {
    return (uint32_t)key % DICT_SIZE;
}

/*
 * PUBLIC
 */

void textureDictionary_init(TextureDictionary_t* this) {
    memset(this->buckets, 0, sizeof(Node_t*)*DICT_SIZE);
}

void textureDictionary_add(TextureDictionary_t* this, TextureID key, const Texture_t* value) {
    uint32_t i = _hash_value(key); //the bucket number this key hashes to
    Node_t* node = this->buckets[i];
    if (node) {
        for (;node->next != NULL && node->key != key; node = node->next);
        if (!node->next) {
            Node_t* new_node = (Node_t*)malloc(sizeof(Node_t));
            _node_init(new_node, key, NULL, value);
            node->next = new_node;
        } else { //the node already existed
            texture_release(&node->texture); //do note that this case should never happen anyway in the code, but the option exists
            node->texture = *value;
        }
    } else {
        Node_t* new_node = (Node_t*)malloc(sizeof(Node_t));
        _node_init(new_node, key, NULL, value);
        this->buckets[i] = new_node;
    }
}

/* Returns NULL if the key is not present in the dictionary */
const Texture_t* textureDictionary_at(TextureDictionary_t* this, TextureID key) {
    uint32_t i = _hash_value(key); //the bucket number this key hashes to
    Node_t* node = this->buckets[i];
    if (!node) return NULL;
    for (;node != NULL && node->key != key; node = node->next);
    if (!node) return NULL;
    return &node->texture;
}

void textureDictionary_release(TextureDictionary_t* this) {
    for (int i = 0; i < DICT_SIZE; i++) {
        if (this->buckets[i]) {
            _node_release(this->buckets[i]); //frees the nodes in the current bucket with recursion
            free(this->buckets[i]);
        }
    }
}
