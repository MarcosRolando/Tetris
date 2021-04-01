//
// Created by marcosrolando on 4/01/21.
//

#include "Texture.h"

/*
 * PRIVATE
 */

void _free(Texture_t* this) {
    if (this->mTexture != NULL) {
        SDL_DestroyTexture(this->mTexture);
        this->mTexture = NULL;
        this->mWidth = 0;
        this->mHeight = 0;
    }
}

/*
 * PUBLIC
 */

void texture_init(Texture_t* this, SDL_Renderer* renderer) {
    this->renderer = renderer;
    this->mTexture = NULL;
    this->mWidth = 0;
    this->mHeight = 0;
    this->xOffset = 0;
    this->yOffset = 0;
    this->defaultScale = 1;
}

void texture_release(Texture_t* this) {
    _free(this);
}

int texture_load_from_file(Texture_t* this, const char* path, ColorKey_t key, int xOff, int yOff, int scale) {
    //Libero la textura anterior
    _free(this);

    //cargo la imagen de path
    SDL_Surface* loadedSurface = IMG_Load(path.c_str());
    if (loadedSurface == NULL) {
        fprintf(stderr, "Unable to load image %s! SDL_image Error: %s\n", path.c_str(), IMG_GetError());
        return TEXTURE_IMAGE_LOADING_ERROR;
    } else {
        if (key.red > -1 && key.green > -1 && key.blue > -1) {
            SDL_SetColorKey(loadedSurface, SDL_TRUE,
                            SDL_MapRGB(loadedSurface->format, key.red, key.green, key.blue));
            /*Con esto aclaras que pixel hacer transparente*/
        }

        //Crea la textura
        this->mTexture = SDL_CreateTextureFromSurface(this->renderer, loadedSurface);
        if (this->mTexture == NULL) {
            //Si falla libero la superficie
            SDL_FreeSurface(loadedSurface);
            fprintf(stderr, "Unable to create texture from %s! Graphics Error: %s\n", path.c_str(), SDL_GetError());
            return TEXTURE_CREATION_ERROR;
        } else {
            this->mWidth = loadedSurface->w;
            this->mHeight = loadedSurface->h;
        }

        //Libero la superficie
        SDL_FreeSurface(loadedSurface);
    }

    this->xOffset = xOff;
    this->yOffset = yOff;
    this->defaultScale = scale;
}



void texture_render(const Texture_t* this, int x, int y, int spritePosition, double angle, int scale) {
    SDL_Rect renderQuad = {x + this->xOffset, y + this->yOffset, this->mWidth, this->mHeight};
    SDL_Rect clip = gSpriteClips.at(spritePosition); //todo ver de implementar esto en C

    //Setea las dimensiones del rectangulo a renderizar
    renderQuad.w = clip.w*scale;
    renderQuad.h = clip.h*scale;

    //Renderiza
    SDL_RenderCopyEx(this->renderer, this->mTexture, &clip, &renderQuad, angle,NULL, SDL_FLIP_NONE);
}

void texture_add_sprite(Texture_t* this, int x, int y, int width, int height) {
    //gSpriteClips.push_back({x, y, width, height}); //todo ver como implementar esto en C
}

SpriteDimensions_t texture_get_sprite_dimesions(const Texture_t* this, int spritePosition) {
    SDL_Rect spriteDimensions = gSpriteClips.at(spritePosition); //todo ver como implementar esto en C
    SpriteDimensions_t dimensions = {spriteDimensions.w, spriteDimensions.h};
    return dimensions;
}
/*
void texture_load_from_rendered_text(Texture_t* this, const char* text, SDL_Color textColor, TTF_Font* font) {
    //Libero la textura anterior
    _free(this);

    //Creo una superficie con el texto
    SDL_Surface* textSurface = TTF_RenderText_Solid(font, text.c_str(), textColor);
    if(textSurface == NULL) {
        fprintf(stderr, "Unable to render text surface! SDL_ttf Error: %s\n", TTF_GetError());
        return TEXTURE_TTF_RENDER_ERROR;
    } else {
        //Crea la textura
        this->mTexture = SDL_CreateTextureFromSurface(this->renderer, textSurface);

        if(this->mTexture == NULL) {
            //Si falla libera la superficie
            SDL_FreeSurface(textSurface);
            fprintf(stderr, "Unable to create texture from rendered text! Graphics Error: %s\n", SDL_GetError());
            return TEXTURE_TTF_CREATION_ERROR;
        } else {
            this->mWidth = textSurface->w;
            this->mHeight = textSurface->h;
            this->gSpriteClips.assign(1, {0, 0, mWidth, mHeight}); //todo ver como implementar esto en C
        }

        //Libero al superficie
        SDL_FreeSurface(textSurface);
    }
}
*/