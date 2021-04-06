//
// Created by marcosrolando on 3/27/21.
//

#include "GameEngine.h"
#include "../view-controller/view/GUI.h"
#include "../view-controller/miscellaneous/MemAllocMacro.h"
#include "../view-controller/controller/Controller.h"
#include <SDL.h>
#include <SDL_image.h>
#include <SDL_mixer.h>
#include <SDL_ttf.h>

/*The following macros where taken from one of my previous projects, and therefore
 * I do not remember if their values matter or not. Probably not. */
#define FREQUENCY 44100
#define CHUNKSIZE 2048

#define SDL_GRAPHICS_ERROR 1
#define SDL_IMAGE_ERROR 2
#define SDL_MIXER_ERROR 3
#define SDL_TTF_ERROR 4

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
static int _initialize_SDL(GameEngine_t* this) {
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
static void _close_SDL(GameEngine_t* this) {
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

int gameEngine_init(GameEngine_t* this) {
    int s = _initialize_SDL(this);
    if (s) return s;
    this->screen = calloc(1, sizeof(Window_t));
    CHECK_ALLOC_RESULT(this->screen,
                       "Failed to allocate memory for the Window! Check you RAM usage or try running again\n")
    s = window_init(this->screen);
    if (s) {
        free(this->screen);
        _close_SDL(this);
        return s;
    }
    this->gui = calloc(1, sizeof(GUI_t));
    CHECK_ALLOC_RESULT(this->gui,
                       "Failed to allocate memory for the GUI! Check you RAM usage or try running again\n")
    s = GUI_init(this->gui, window_get_renderer(this->screen));
    if (s) {
        window_release(this->screen);
        free(this->screen);
        free(this->gui);
        _close_SDL(this);
    }
    this->controller = calloc(1, sizeof(Controller_t));
    CHECK_ALLOC_RESULT(this->screen,
                       "Failed to allocate memory for the Controller! Check you RAM usage or try running again\n")
    controller_init(this->controller);
    return s;
}

Input_t gameEngine_read_event(const GameEngine_t* this) {
    return controller_read_event(this->controller, this->screen);
}

void gameEngine_render(const GameEngine_t* this, const GameState_t* game_state) {
    window_clear(this->screen);
    GUI_render(this->gui, game_state);
    window_show(this->screen);
}

void gameEngine_release(GameEngine_t* this) {
    GUI_release(this->gui);
    controller_release(this->controller);
    window_release(this->screen);
    free(this->gui);
    free(this->screen);
    free(this->controller);
    _close_SDL(this);
}
