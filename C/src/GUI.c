//
// Created by marcosrolando on 4/2/21.
//

#include "GUI.h"

int GUI_init(GUI_t* this) {
    int s = window_init(&this->screen);
    if (s) return s;
    s = textureRepository_init(&this->texture_repo, window_get_renderer(&this->screen));
    if (s) window_release(&this->screen);
    return s;
}

void GUI_render(const GUI_t* this) {
    window_clear(&this->screen);
    const Texture_t* tiles = textureRepository_get_texture(&this->texture_repo, BACKGROUNDS);
    texture_render(tiles, 0, 0, 0, 0, SCREEN_SCALE);
    texture_render(textureRepository_get_texture(&this->texture_repo, TILES),
                   96*SCREEN_SCALE, 192*SCREEN_SCALE, 5, 0, SCREEN_SCALE); //Classic NES tetris leaves one pixel next to the lateral borders and two pixels next to the base
    window_show(&this->screen);
}

void GUI_release(GUI_t* this) {
    textureRepository_release(&this->texture_repo);
    window_release(&this->screen);
}
