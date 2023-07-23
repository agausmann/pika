add_library(libcpika
    fnv.c
    intern.c
)
target_include_directories(libcpika PUBLIC .)

add_executable(cpika
    cpika.c
)
target_link_libraries(cpika libcpika)
