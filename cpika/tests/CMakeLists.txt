add_executable(check_intern check_intern.c)
target_link_libraries(check_intern libcpika)
add_test(NAME check_intern COMMAND check_intern)
