cmake_minimum_required(VERSION 3.17)
project(tetris)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wall -Werror -pedantic -pedantic-errors -O0 -ggdb -DDEBUG -fno-inline -lm -pthread")

find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

add_library(viewer src/Viewer.c src/Viewer.h src/Window.c src/Window.h src/ViewUnit.c src/Texture.c src/Texture.h src/Vector.c src/Vector.h)
target_link_libraries(viewer SDL2 SDL2_image SDL2_ttf SDL2_mixer)
