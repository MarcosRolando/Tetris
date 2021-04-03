//
// Created by marcosrolando on 4/01/21.
//

#include "TextureRepository.h"

#define TILES_PATH "/home/marcosrolando/CLionProjects/Tetris/C/assets/tiles.png"

/*
 * PRIVATE
 */

static int _load_tiles(TextureRepository_t* this, const char* image_path) {
    Texture_t tile_texture;
    texture_init(&tile_texture, this->renderer);
    ColorKey_t color_key = {-1, -1, -1};
    int s = texture_load_from_file(&tile_texture, image_path, color_key, 0, 0, 1);
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

/*
 * PUBLIC
 */

int textureRepository_init(TextureRepository_t* this, SDL_Renderer* renderer) {
    textureDictionary_init(&this->textures);
    this->renderer = renderer;
    return _load_tiles(this, TILES_PATH);
    //todo cargar las otras cosas
}

const Texture_t* textureRepository_get_texture(const TextureRepository_t* this, TextureID texture_key) {
    return textureDictionary_at(&this->textures, texture_key);
}

void textureRepository_release(TextureRepository_t* this) {
    textureDictionary_release(&this->textures);
}

/*
void TextureRepository::_loadUI() {
    _setImage(Background, BACKGROUND_PATH, 1495, 937, 0
            , 0, 1, {-1, -1, -1});
    _setImage(MainMenu, MAIN_MENU_PATH, 1499, 937, 0,
            0, 1, {-1, -1, -1});
}

void TextureRepository::_setTileImage(TextureID TextureID, std::string&& tileImage, bool individualTile) {
    try {
        textures.emplace(TextureID, renderer);
        Texture& texture = textures.at(TextureID);
        texture.loadFromFile(tileImage);
        _addTileSprites(texture, 0, individualTile);
    } catch (TPException& e) {
        throw TPException("Failed to load %s sprite sheet texture!\n", tileImage.c_str());
    }
}

void TextureRepository::_loadTiles() {
    _setTileImage(Grass, GRASS_PATH, false);
    _setTileImage(PrettyGrass, PRETTY_GRASS_PATH, false);
    _setTileImage(PrettyRoad, PRETTY_ROAD_PATH, false);
    _setTileImage(DeadGrass, DEAD_GRASS_PATH, false);
    _setTileImage(Water, WATER_PATH, false);
    _setTileImage(DarkWater, DARK_WATER_PATH, false);
    _setTileImage(Sand, SAND_PATH, true);
}

void TextureRepository::_addTileSprites(Texture& texture, int y, bool individualTile) {
    texture.addSprite(0, 0, TILE_WIDTH, TILE_HEIGHT);
    if (!individualTile) {
        texture.addSprite(TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT);
        texture.addSprite(2*TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT);
        texture.addSprite(3*TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT);
    }
}

void TextureRepository::_addSprites(Texture& texture, int width, int height) {
    texture.addSprite(0, 0, width, height);
}
 */
