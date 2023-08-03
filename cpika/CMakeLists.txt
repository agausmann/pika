cmake_minimum_required(VERSION 3.26)
project(cpika)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake/ECM/find-modules")

include(CTest)

find_package(BISON)
find_package(FLEX)
find_package(Gperf)

add_subdirectory(src)
add_subdirectory(tests)
