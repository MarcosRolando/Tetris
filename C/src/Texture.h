//
// Created by marcosrolando on 4/01/21.
//

#ifndef TETRIS_TEXTURE_H
#define TETRIS_TEXTURE_H

/* Represents a loaded texture. A texture may have more than one sprite, so you can
 * select which sprite to access */

#include <SDL.h>
#include <SDL_image.h>
#include "Vector.h"

#define TEXTURE_IMAGE_LOADING_ERROR 1
#define TEXTURE_CREATION_ERROR 2

typedef struct ColorKey {
    int red;
    int green;
    int blue;
} ColorKey_t;

typedef struct SpriteDimensions {
    int width;
    int height;
} SpriteDimensions_t;

typedef struct Texture {
    SDL_Renderer* renderer;
    SDL_Texture* mTexture;
    int mWidth;
    int mHeight;
    int xOffset;
    int yOffset;
    int defaultScale;
    Vector_t/*<SDL_Rect>*/ g_sprite_clips; /*Sprites de la textura*/
} Texture_t;

void texture_init(Texture_t* this, SDL_Renderer* renderer);

void texture_release(Texture_t* this);

/*Carga la imagen de path, ignorando el color recibido en key. Opcionalmente
* se le puede setear un offset de renderizacion y una escala distinta a la
* imagen*/
int texture_load_from_file(Texture_t* this, const char* path, ColorKey_t key, int xOff,
                            int yOff, int scale);

/*Especifica una dimension (un clip) que representa un sprite de la textura*/
void texture_add_sprite(Texture_t* this, int x, int y, int width, int height);

/*Renderiza el sprite de la textura en la posicion, angulo y escala indicados*/
void texture_render(const Texture_t* this, int x, int y, int spritePosition, double angle, int scale);

/*Retorna las dimensiones del sprite de la textura*/
SpriteDimensions_t texture_get_sprite_dimesions(const Texture_t* this, int spritePosition);

/*Crea una textura en base al texto recibido*/
//void texture_load_from_rendered_text(Texture_t* this, const char* text, SDL_Color textColor, TTF_Font* font);

#endif //TETRIS_TEXTURE_H
