cmake_minimum_required(VERSION 3.17)
project(tetris)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wall -Wconversion -Werror -pedantic -pedantic-errors -O0 -ggdb -DDEBUG -fno-inline -lm -pthread")

find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

add_library(viewer src/Viewer.c src/Viewer.h src/Window.c src/Window.h src/ViewUnit.c src/Texture.c src/Texture.h
            src/Vector.c src/Vector.h src/TextureRepository.h src/TextureRepository.c src/TextureDictionary.c
        src/TextureDictionary.h src/TextureID.h src/MemAllocMacro.h src/GUI.c src/GUI.h src/Board.c src/Board.h src/GameState.h)
target_link_libraries(viewer SDL2 SDL2_image SDL2_ttf SDL2_mixer)
