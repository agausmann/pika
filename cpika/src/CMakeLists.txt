add_subdirectory(core)

add_executable(cpika
    cpika.c
)
target_link_libraries(cpika cpika-core)
