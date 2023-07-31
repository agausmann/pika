cmake_minimum_required(VERSION 3.26)
project(cpika)

include(CTest)

find_package(BISON)
find_package(FLEX)

add_subdirectory(src)
add_subdirectory(tests)
