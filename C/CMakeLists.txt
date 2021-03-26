cmake_minimum_required(VERSION 3.17)
project(foo)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c99 -Wall -Werror -pedantic -pedantic-errors -O0 -ggdb -DDEBUG -fno-inline -lm -pthread")

find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

add_library(foo src/foo.h src/foo.c)
