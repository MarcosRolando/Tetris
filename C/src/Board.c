//
// Created by marcosrolando on 4/3/21.
//

#include "Board.h"
#include "TextureRepository.h"
#include "Window.h"

void board_init(Board_t* this, const TextureRepository_t* texture_repo) {
    this->texture_repo = texture_repo;
}

void board_render(const Board_t* this, const GameState_t* game_state) {
    const Texture_t* texture = textureRepository_get_texture(this->texture_repo, BACKGROUNDS);
    texture_render(texture, 0, 0, 0, 0, SCREEN_SCALE); // We render the background
    texture = textureRepository_get_texture(this->texture_repo, TILES);
    for (int i = 0; i < BOARD_HEIGHT; i++) {
        for (int j = 0; j < BOARD_WIDTH; j++) {
            if (game_state->board_config[i][j] != NONE) { //todo implementar la seleccion de color de acuerdo al tipo de pieza
                texture_render(texture,(96+j)*SCREEN_SCALE, (192+i)*SCREEN_SCALE, 0, 0, SCREEN_SCALE); //Classic NES tetris leaves one pixel next to the lateral borders and two pixels next to the base
            }
        }
    }
}

void board_release(Board_t* this) {
    //do nothing
}
