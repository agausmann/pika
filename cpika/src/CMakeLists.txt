BISON_TARGET(
    cpika pika.y ${CMAKE_CURRENT_BINARY_DIR}/pika.tab.c
    DEFINES_FILE ${CMAKE_CURRENT_BINARY_DIR}/pika.tab.h
    COMPILE_FLAGS -Wcounterexamples
)

FLEX_TARGET(
    cpika pika.l ${CMAKE_CURRENT_BINARY_DIR}/pika.yy.c
    DEFINES_FILE ${CMAKE_CURRENT_BINARY_DIR}/pika.yy.h
)

add_library(libcpika
    fnv.c
    intern.c
    lex.c
    parse.c
    ${BISON_cpika_OUTPUTS}
    ${FLEX_cpika_OUTPUTS}
)
target_include_directories(libcpika PUBLIC . ${CMAKE_CURRENT_BINARY_DIR})

add_executable(cpika
    cpika.c
)
target_link_libraries(cpika libcpika)
