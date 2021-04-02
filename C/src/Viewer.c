//
// Created by marcosrolando on 3/26/21.
//

#include "Viewer.h"
#include <stdio.h>
#include <SDL.h>
#include <SDL_image.h>
#include <SDL_mixer.h>
#include <SDL_ttf.h>

/*The following macros where taken from one of my previous projects, and therefore
 * I do not remember if their values matter or not. Probably not. */
#define FREQUENCY 44100
#define CHUNKSIZE 2048

/*
 * PRIVATE
 */

/*
 * Starts SDL Systems
 * If there is a graphics error it returns SDL_GRAPHICS_ERROR
 * If there is an image loader error it returns SDL_IMG_ERROR
 * If there is an audio error it returns SDL_MIXER_ERROR
 * If there is a text font loader error it returns SDL_TTF_ERROR
 * Otherwise returns 0
 */
static int _initialize_SDL(Viewer_t* this) {
    //Starts video and audio
    if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO) < 0) {
        fprintf(stderr, "Graphics could not initialize! Graphics Error: %s\n", SDL_GetError());
        return SDL_GRAPHICS_ERROR;
    } else {
        //Sets linear texture filtering
        if (!SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "MipmapLinearNearest")) {
            fprintf(stderr, "Warning: Linear texture filtering not enabled!");
        }
        //Starts PNG loading
        int imgFlags = IMG_INIT_PNG;
        if (!(IMG_Init(imgFlags) & imgFlags)) {
            SDL_Quit();
            fprintf(stderr, "SDL_image could not initialize! SDL_image Error: %s\n", IMG_GetError());
            return SDL_IMAGE_ERROR;
        }
    }
    //Starts audio, allows MP3 files
    if (Mix_Init(MIX_INIT_MP3) == 0) {
        IMG_Quit();
        SDL_Quit();
        fprintf(stderr, "SDL_mixer could not initialize! SDL_mixer Error: %s\n", Mix_GetError());
        return SDL_MIXER_ERROR;
    }
    //Starts audio player
    if (Mix_OpenAudio(FREQUENCY, MIX_DEFAULT_FORMAT, 2, CHUNKSIZE) < 0) {
        Mix_Quit();
        IMG_Quit();
        SDL_Quit();
        fprintf(stderr, "SDL_mixer could not initialize! SDL_mixer Error: %s\n", Mix_GetError());
        return SDL_MIXER_ERROR;
    }
    //Starts text font loader
    if (TTF_Init() == -1) {
        Mix_Quit();
        IMG_Quit();
        SDL_Quit();
        fprintf(stderr, "SDL_ttf could not initialize! SDL_ttf Error: %s\n", TTF_GetError());
        return SDL_TTF_ERROR;
    }
    return 0;
}

/* Releases SDL Systems */
static void _close_SDL(Viewer_t* this) {
    TTF_Quit();
    Mix_CloseAudio();
    Mix_Quit();
    IMG_Quit();
    SDL_QuitSubSystem(SDL_INIT_VIDEO | SDL_INIT_AUDIO);
    SDL_Quit();
}

/*
 * PUBLIC
 */

int viewer_init(Viewer_t* this) {
    int s = _initialize_SDL(this);
    if (s) return s;
    s = GUI_init(&this->gui);
    if (s) _close_SDL(this);
    return s;
}

void viewer_render_frame(const Viewer_t* this) {
    GUI_render(&this->gui);
}

void viewer_release(Viewer_t* this) {
    GUI_release(&this->gui);
    _close_SDL(this);
}
