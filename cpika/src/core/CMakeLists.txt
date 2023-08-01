BISON_TARGET(
    cpika pika.y ${CMAKE_CURRENT_BINARY_DIR}/pika.tab.c
    DEFINES_FILE ${CMAKE_CURRENT_BINARY_DIR}/pika.tab.h
    COMPILE_FLAGS -Wcounterexamples
)

FLEX_TARGET(
    cpika pika.l ${CMAKE_CURRENT_BINARY_DIR}/pika.yy.c
    DEFINES_FILE ${CMAKE_CURRENT_BINARY_DIR}/pika.yy.h
)

add_library(cpika-core
    fnv.c
    intern.c
    lex.c
    parse.c
    ${BISON_cpika_OUTPUTS}
    ${FLEX_cpika_OUTPUTS}
)
target_include_directories(cpika-core PUBLIC . ${CMAKE_CURRENT_BINARY_DIR})
