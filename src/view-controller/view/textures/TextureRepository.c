//
// Created by marcosrolando on 4/01/21.
//

#include "TextureRepository.h"

#define TILES_PATH "/home/marcos/Tetris/assets/tiles.png"
#define BACKGROUND_PATH "/home/marcos/Tetris/assets/backgrounds.png"


/*
 * PRIVATE
 */

static int _load_tiles(TextureRepository_t* this) {
    Texture_t tile_texture;
    texture_init(&tile_texture, this->renderer);
    ColorKey_t color_key = {-1, -1, -1};
    int s = texture_load_from_file(&tile_texture, TILES_PATH, color_key, 0, 0, 1);
    if (s) {
        texture_release(&tile_texture);
        return s;
    }
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 4; j++) {
            texture_add_sprite(&tile_texture, j*8, i*8, 8, 8); //todo ver si las dimensiones estan bien
        }
    }
    textureDictionary_add(&this->textures, TILES, &tile_texture);
    texture_release(&tile_texture);
    return 0;
}

static int _load_backgrounds(TextureRepository_t* this) {
    Texture_t background_texture;
    texture_init(&background_texture, this->renderer);
    ColorKey_t color_key = {-1, -1, -1};
    int s = texture_load_from_file(&background_texture, BACKGROUND_PATH, color_key, 0, 0, 1);
    if (s) {
        texture_release(&background_texture);
        return s;
    }
    texture_add_sprite(&background_texture, 7, 6, 256, 224); //these magic numbers match the backgrounds.png sprite to perfection!
    texture_add_sprite(&background_texture, 268, 6, 256, 224); //these magic numbers match the backgrounds.png sprite to perfection!
    textureDictionary_add(&this->textures, BACKGROUNDS, &background_texture);
    texture_release(&background_texture);
    return 0;
}

/*
 * PUBLIC
 */

int textureRepository_init(TextureRepository_t* this, SDL_Renderer* renderer) {
    textureDictionary_init(&this->textures);
    this->renderer = renderer;
    int s = _load_tiles(this);
    if (s) return s;
    return _load_backgrounds(this);
    //todo cargar las otras cosas
}

const Texture_t* textureRepository_get_texture(const TextureRepository_t* this, TextureID texture_key) {
    return textureDictionary_at(&this->textures, texture_key);
}

void textureRepository_release(TextureRepository_t* this) {
    textureDictionary_release(&this->textures);
}
