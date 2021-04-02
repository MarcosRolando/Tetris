//
// Created by marcosrolando on 4/01/21.
//

#ifndef ARGENTUM_TEXTUREREPOSITORY_H
#define ARGENTUM_TEXTUREREPOSITORY_H

#include "Texture.h"

typedef enum TextureID {
    A,
    B,
    C //todo poner los valores reales de las texturas
} TextureID;

typedef struct TextureRepository {
    TextureDictonary_t textures; /*The keys are TextureID enums and the values are Textures */
    SDL_Renderer* renderer;
} TextureRepository_t;

void textureRepository_init(TextureRepository_t* this, SDL_Renderer* renderer);

const Texture_t* textureRepository_get_texture(const TextureRepository_t* this, TextureID texture);

void textureRepository_release(TextureRepository_t* this);


#endif //ARGENTUM_TEXTUREREPOSITORY_H
