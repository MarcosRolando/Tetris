cmake_minimum_required(VERSION 3.17)
project(tetris)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wall -Wconversion -Werror -pedantic -pedantic-errors -O0 -ggdb -DDEBUG -fno-inline -lm -pthread")

find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

add_library(gameEngine src/view-controller/view/Window.c src/view-controller/view/Window.h src/common/GameEngine.c
        src/view-controller/view/textures/Texture.c src/view-controller/view/textures/Texture.h
        src/view-controller/miscellaneous/Vector.c src/view-controller/miscellaneous/Vector.h
        src/view-controller/view/textures/TextureRepository.h src/view-controller/view/textures/TextureRepository.c
        src/view-controller/view/textures/TextureDictionary.c
        src/view-controller/view/textures/TextureDictionary.h src/view-controller/view/textures/TextureID.h
        src/view-controller/miscellaneous/MemAllocMacro.h src/view-controller/view/GUI.c
        src/view-controller/view/GUI.h src/view-controller/view/Board.c src/view-controller/view/Board.h
        src/common/GameState.h src/view-controller/controller/Controller.c src/view-controller/controller/Controller.h)

target_link_libraries(gameEngine SDL2 SDL2_image SDL2_ttf SDL2_mixer)
