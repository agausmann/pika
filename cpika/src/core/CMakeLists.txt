set(CPIKA_GENERATED_HEADERS_DIR ${CMAKE_CURRENT_BINARY_DIR}/include)

BISON_TARGET(
    cpika ${CMAKE_CURRENT_SOURCE_DIR}/pika.y ${CMAKE_CURRENT_BINARY_DIR}/pika.tab.c
    DEFINES_FILE ${CPIKA_GENERATED_HEADERS_DIR}/pika.tab.h
    COMPILE_FLAGS -Wcounterexamples
)

FLEX_TARGET(
    cpika ${CMAKE_CURRENT_SOURCE_DIR}/pika.l ${CMAKE_CURRENT_BINARY_DIR}/pika.yy.c
    DEFINES_FILE ${CPIKA_GENERATED_HEADERS_DIR}/pika.yy.h
)

add_library(cpika-core
    fnv.c
    intern.c
    parse.c
    parser_context.c
    ${BISON_cpika_OUTPUTS}
    ${FLEX_cpika_OUTPUTS}
)
target_include_directories(cpika-core PUBLIC . ${CPIKA_GENERATED_HEADERS_DIR})

ecm_gperf_generate(
    ${CMAKE_CURRENT_SOURCE_DIR}/keywords.gperf
    ${CPIKA_GENERATED_HEADERS_DIR}/keywords.h
    cpika-core
)
