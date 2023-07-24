include(CTest)

add_executable(check_intern check_intern.c)
target_link_libraries(check_intern libcpika)
add_test(NAME check_intern COMMAND check_intern)

add_executable(check_test check_test.c)
add_test(NAME check_test COMMAND check_test)
set_property(TEST check_test PROPERTY WILL_FAIL true)
