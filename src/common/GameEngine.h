//
// Created by marcosrolando on 3/27/21.
//

#ifndef TETRIS_GAMEENGINE_H
#define TETRIS_GAMEENGINE_H

#include "GameState.h"


/*
 * This unit acts as a translator between the Model code and the view-controller code, the latter being the one
 * who handles all graphics/input related aspects. This provides a safe API for the C code that Rust can use.
 */

struct GUI;
struct Controller;
struct Window;

//The data are pointers because if not then bindgen breaks when trying to translate GameEngine.h to
//game_engine.rs because I would have to include these classes which would also includes SDL.
typedef struct GameEngine {
    struct GUI* gui; //This handles the graphics
    struct Controller* controller; //This handles the input of the player
    struct Window* screen; //The window where the game is shown
} GameEngine_t;

/* Constructor */
int gameEngine_init(GameEngine_t* this);

Input_t gameEngine_read_event(const GameEngine_t* this);

/* Renders the frame based on the current state of the game */
void gameEngine_render(const GameEngine_t* this, const GameState_t* game_state);

/* Destructor */
void gameEngine_release(GameEngine_t* this);

#endif //TETRIS_GAMEENGINE_H
