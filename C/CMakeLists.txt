cmake_minimum_required(VERSION 3.17)
project(foo)

set(CMAKE_C_STANDARD 99)

add_library(foo src/foo.h src/foo.c)
